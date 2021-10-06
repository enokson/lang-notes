use std::{sync::MutexGuard};

use actix_web::{ web::{ Data, Query }, HttpResponse };
use postgres::{ Client as Db };
use serde::{ Deserialize, Serialize };
use super::super::{AppData};

#[derive(Deserialize)]
pub struct Info {
    pub text: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    id: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Reply {
    Ok {
        data: Vec<Cluster>
    },
    Error {
        error: String
    }
}

fn get_cluster_rows(mut db: MutexGuard<Db>, text: &str) -> Result<Vec<Cluster>, String> {
    let sql = include_str!("select_keys.sql");
    let raw_rows = { 
        match db.query(sql, &[&text]) {
            Ok(rows) => Ok(rows),
            Err(error) => Err(error.to_string())
        }
    }?;
    let rows = {
        let result: Result<Vec<_>, _> = raw_rows
        .iter()
        .map(|row| -> Result<Cluster, String> {
            let id = {
                match row.try_get("id") {
                    Ok(v) => Ok(v),
                    Err(error) => Err(error.to_string())
                }
            }?;
            Ok(Cluster { id })
        }).collect();
        result
    }?;
    Ok(rows)
}

pub fn get_clusters_by_search_keys(data: Data<AppData>, query: Query<Info>) -> HttpResponse {
    let get_reply = || -> Result<HttpResponse, HttpResponse> {
        let s = |txt: &str| txt.to_string();
        let db = {
            match data.db.lock() {
                Ok(db) => Ok(db),
                Err(error) => {
                    Err(
                        HttpResponse::InternalServerError().json(Reply::Error { error: s("internal server error")  })
                    )
                }
            }
        }?;
        match get_cluster_rows(db, &query.text) {
            Ok(data) => Ok(HttpResponse::Ok().json(Reply::Ok { data })),
            Err(error) => Err(HttpResponse::InternalServerError().json(Reply::Error{ error }))
        }  
    };
    match get_reply() {
        Ok(reply) => reply,
        Err(reply) => reply
    }
}