use actix_web::web::{Data, HttpResponse, Query};
use serde::{Deserialize};
use crate::{
    AppData,
    api::{
        example::{
            get_example::get_example_from_row,
            delete_example::delete_example
        }
    },
    schema::{
        Int,
        ParamIndexer,
        examples::{
            table as examples_table
        },
        translations::{
            table
        }
    },
    error_msg
};

#[derive(Debug, Deserialize)]
pub struct Info {
    id: Int
}

pub fn delete_translation(data: &Data<AppData>, id: &i32) -> Result<(), String> {
    let mut db = error_msg!(data.db.try_lock())?;
    {
        let mut indexer = ParamIndexer::new();
        let sql = vec![
            "select", examples_table::ID,
            "from", examples_table::TABLE_NAME,
            "where", examples_table::PARENT_ID, "=", &indexer.next(),
            "AND", examples_table::PARENT_TYPE, "=", &indexer.next()
        ].join(" ");
        match error_msg!(db.query(sql.as_str(), &[id, examples_table::PARENT_TYPE_TRANSLATION])) {
            Ok(rows) => {
                for row in rows.iter() {
                    let example = error_msg!(get_example_from_row(row))?;
                    error_msg!(delete_example(&data, &example.id))?;
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
    match error_msg!(delete_translation(&data, &info.id)) {
        Ok(_) => {
            return HttpResponse::Ok().finish();
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    }  
}
