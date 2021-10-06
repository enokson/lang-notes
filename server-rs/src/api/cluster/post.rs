use actix_web::{
    web::{
        Data,
        Json
    },
    HttpResponse
};
use serde::{Serialize};
use super::super::super::{
    AppData,
    schema::{ get_db, clusters}
};

#[derive(Debug, Serialize)]
pub enum Reply {
    Ok { id: u32 },
    Err { error: String }
}

pub fn post(data: Data<AppData>, cluster: Json<clusters::NewCluster>) -> HttpResponse {
    let get_reply = || -> Result<HttpResponse, HttpResponse> {
        let mut db = match get_db(&data.db) {
            Ok(db) => Ok(db),
            Err(_) => Err(HttpResponse::InternalServerError().json(Reply::Err{ error: "internal server error".to_string() }))
        }?;
        let cluster_id = {
            match clusters::insert_cluster_row(&mut db) {
                Ok(id) => Ok(id),
                Err(error) => Err(HttpResponse::InternalServerError().json(Reply::Err{ error }))
            }
        }?;
        Ok(HttpResponse::Ok().json(Reply::Ok{ id: 1 }))
    };
    match get_reply() {
        Ok(reply) => reply,
        Err(reply) => reply
    }
}
