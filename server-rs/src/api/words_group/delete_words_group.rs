use actix_web::web::{Data, HttpResponse, Query};
use serde::{Deserialize};
use crate::{
    AppData,
    api::{
        definition::{
            get_definition::get_definition_from_row,
            delete_definition::delete_definition
        }
    },
    schema::{
        Db,
        ParamIndexer,
        definitions::table as definitions_table,
        word_groups::{
            table
        }
    },
    error_msg
};

#[derive(Debug, Deserialize)]
pub struct Info {
    id: i32
}

pub fn delete_words_group(db: &mut Db, id: &i32) -> Result<(), String> {
    let mut indxer = ParamIndexer::new();
    {
        let mut indexer = ParamIndexer::new();
        let sql = vec![
            "select", definitions_table::ID, 
            "from", definitions_table::TABLE_NAME,
            "where", definitions_table::CLUSTER_ID, "=", &indexer.next()
        ].join(" ");
        match error_msg!(db.query(sql.as_str(), &[id])) {
            Ok(rows) => {
                for postgres_row in rows.iter() {
                    let definition_row = error_msg!(get_definition_from_row(&postgres_row))?;
                    error_msg!(delete_definition(db, &definition_row.id))?
                }
            },
            Err(error) => {
                return Err(error);
            }
        };
    }
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
        Ok(mut db) => match error_msg!(delete_words_group(&mut db, &info.id)) {
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
