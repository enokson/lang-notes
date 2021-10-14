use actix_web::{
    web::{Data, Json},
    HttpResponse
};
use postgres::types::ToSql;
use serde::{ Serialize };
use crate::{
    AppData,
    schema::{
        Db,
        examples::{table, RowI}
    },
    error_msg
};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Reply {
    Ok { id: i32 },
    Err { error: String }
}

pub fn post_example(db: &mut Db, body: &RowI) -> Result<i32, String> {
    let columns = vec![
        table::PARENT_TYPE,
        table::PARENT_ID,
        table::EXAMPLE
    ];
    let columns_str = columns.join(",");
    let sql = format!("insert into {} ({}) values ($1, $2, $3)", table::TABLE_NAME, columns_str);
    let params: Vec<Box<(dyn ToSql + Sync)>> = vec![
        Box::new(body.parent_type),
        Box::new(body.parent_id),
        Box::new(body.example.clone())
    ];
    let id = match error_msg!(db.query(sql.as_str(), &params.iter().map(|p| p.as_ref()).collect::<Vec<&(dyn ToSql + Sync)>>())) {
        Ok(rows) => match rows.get(0) {
            Some(row) => match error_msg!(row.try_get("id")) {
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
        Ok(mut db) => match error_msg!(post_example(&mut db, &body.into_inner())) {
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
