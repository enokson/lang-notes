use actix_web::web::{Data, HttpResponse, Query};
use serde::{Deserialize};
use crate::{
    AppData,
    api::{
        example::{
            get_example::get_example_from_row,
            delete_example::delete_example
        },
        translation::{
            get_translation::get_translation_from_row,
            delete_translation::delete_translation
        }
    },
    schema::{
        Db,
        ParamIndexer,
        definitions::{
            table
        },
        examples::{
            table as examples_table
        },
        translations::{
            table as translations_table
        }
    },
    error_msg
};

#[derive(Debug, Deserialize)]
pub struct Info {
    id: i32
}

pub fn delete_definition(db: &mut Db, id: &i32) -> Result<(), String> {
    {
        let mut indexer = ParamIndexer::new();
        let sql = vec![
            "select", examples_table::ID,
            "from", examples_table::TABLE_NAME,
            "where", examples_table::PARENT_ID, "=", &indexer.next(),
            "AND", examples_table::PARENT_TYPE, "=", &indexer.next()
        ].join(" ");
        match error_msg!(db.query(sql.as_str(), &[id, examples_table::PARENT_TYPE_DEFINITION])) {
            Ok(rows) => {
                for row in rows.iter() {
                    let example = error_msg!(get_example_from_row(row))?;
                    error_msg!(delete_example(db, &example.id))?;
                }
            },
            Err(error) => {
                return Err(error);
            }
        }
    }
    {
        let mut indexer = ParamIndexer::new();
        let sql = vec![
            "select", translations_table::ID,
            "from", translations_table::TABLE_NAME,
            "where", translations_table::DEFINITION_ID, "=", &indexer.next()
        ].join(" ");
        match error_msg!(db.query(sql.as_str(), &[id])) {
            Ok(rows) => {
                for row in rows.iter() {
                    let translation = error_msg!(get_translation_from_row(row))?;
                    error_msg!(delete_translation(db, &translation.id))?;
                }
            },
            Err(error) => {
                return Err(error);
            }
        }
    }
    let mut indxer = ParamIndexer::new();
    let sql = vec![
        "delete", "from", table::TABLE_NAME,
        "where", table::ID, "=", &indxer.next()
    ].join(" ");
    match error_msg!(db.query(sql.as_str(), &[id])) {
        Ok(_rows) => Ok(()),
        Err(error) => Err(error)
    }
}

pub fn delete(data: Data<AppData>, info: Query<Info>) -> HttpResponse {
    match error_msg!(data.db.try_lock()) {
        Ok(mut db) => match error_msg!(delete_definition(&mut db, &info.id)) {
            Ok(_) => {
                return HttpResponse::Ok().finish();
            },
            Err(error) => {
                println!("{}", error);
                return HttpResponse::InternalServerError().finish();
            }
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    };
}
