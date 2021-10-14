use actix_web::web::{Data, HttpResponse, Query};
use serde::{Deserialize};
use crate::{
    AppData, 
    schema::{
        Db,
        ParamIndexer,
        languages::{
            table
        }
    },
    error_msg
};

#[derive(Debug, Deserialize)]
pub struct Info {
    id: i32
}

pub fn delete_lang(db: &mut Db, id: &i32) -> Result<(), String> {
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
        Ok(mut db) => match error_msg!(delete_lang(&mut db, &info.id)) {
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
