use actix_web::{
    web::{Data, Json},
    HttpResponse
};
use crate::error_msg;

use super::super::super::{
    AppData, schema,
};
use schema::{
    Db,
    ParamIndexer,
    examples::{table, Row}
};

pub fn update_example(db: &mut Db, body: &Row) -> Result<(), String> {
    let mut indx = ParamIndexer::new();
    let updates = vec![
        "set", table::PARENT_TYPE, "=", &indx.next(),
        "set", table::PARENT_ID, "=", &indx.next(),
        "set", table::EXAMPLE, "=", &indx.next()
    ].join(",");
    let sql = vec![
        "update", table::TABLE_NAME,
        "where", table::ID, "=", &indx.next(),
        &updates        
    ].join(" ");
    match error_msg!(db.query(sql.as_str(), &[&body.id, &body.parent_type, &body.parent_id, &body.example])) {
        Ok(_rows) => Ok(()),
        Err(error) => Err(error)
    }
}

pub fn update(data: Data<AppData>, body: Json<Row>) -> HttpResponse {
    match error_msg!(data.db.try_lock()) {
        Ok(mut db) => match error_msg!(update_example(&mut db, &body.into_inner())) {
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
