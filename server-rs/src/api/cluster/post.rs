
use actix_web::{
    web::{
        Data,
        Json
    },
    HttpResponse
};
use serde::{ Deserialize, Serialize};
use super::super::super::{
    AppData,
    schema::{ 
        get_db, 
        clusters
    }
};

#[derive(Debug, Deserialize)]
pub struct Body {
    
}

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
        let id = {
            match data.clusters.insert_one(&mut db, &vec![]) {
                Ok(id) => Ok(id),
                Err(error) => Err(HttpResponse::InternalServerError().json(Reply::Err{ error }))
            }
        }?;
        // TODO: insert remaining data fields
        Ok(HttpResponse::Ok().json(Reply::Ok{ id }))
    };
    match get_reply() {
        Ok(reply) => reply,
        Err(reply) => reply
    }
}
