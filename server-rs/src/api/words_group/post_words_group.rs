use actix_web::{
    web::{Data, Json},
    HttpResponse
};
use serde::{ Serialize };
use crate::{
    AppData,
    schema::{
        Db,
        ParamIndexer,
        word_groups::{table, RowI}
    },
    error_msg
};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Reply {
    Ok { id: i32 },
    Err { error: String }
}

pub fn post_words_group(db: &mut Db, body: &RowI) -> Result<i32, String> {
    let mut indexer = ParamIndexer::new();
    let sql = vec![
        "insert", "into", table::TABLE_NAME,
        &format!("({})", vec![table::NAME].join(",")),
        &format!("values ({})", indexer.params(&1)),
        "returning", table::ID
    ].join(" ");
    let id = match error_msg!(db.query(sql.as_str(), &[&body.name])) {
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

pub fn post(data: Data<AppData>, body: Json<RowI>) -> HttpResponse {
    match error_msg!(data.db.try_lock()) {
        Ok(mut db) => match error_msg!(post_words_group(&mut db, &body.into_inner())) {
            Ok(id) => {
                return HttpResponse::Ok().json(Reply::Ok{ id });
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
