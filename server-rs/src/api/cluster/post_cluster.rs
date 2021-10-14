use actix_web::{
    web::Data,
    HttpResponse
};
use serde::Serialize;
use crate::{
    AppData,
    schema::{
        clusters::table
    },
    error_msg
};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Reply {
    Ok { id: i32 },
    Err { error: String }
}

pub fn post_cluster(data: Data<AppData>) -> Result<i32, String> {
    let mut db = error_msg!(data.db.try_lock())?;
    let sql = vec![
        "insert", "into", table::TABLE_NAME,
        "values", "(default)",
        "returning", table::ID
    ].join(" ");
    let id = match error_msg!(db.query(sql.as_str(), &[])) {
        Ok(rows) => match rows.get(0) {
            Some(row) => match error_msg!(row.try_get(table::ID)) {
                Ok(id) => id,
                Err(error) => {
                    return Err(error);
                }
            },
            None => {
                return Err(format!("{}::{} Could not find row.", file!(), line!()));
            }
        },
        Err(error) => {
            return Err(error);
        }
    };
    return Ok(id);
}

pub fn post(data: Data<AppData>) -> HttpResponse {
    match error_msg!(post_cluster(data)) {
        Ok(id) => {
            return HttpResponse::Ok().json(Reply::Ok{ id });
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
