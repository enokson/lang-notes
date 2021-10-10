#![allow(dead_code)]

use actix_web::{ web, App, HttpServer, Responder };
use postgres::{ Client, NoTls };
use std::{
    sync::{ Mutex }
};

mod api;
mod schema;

pub struct AppData {
    pub db: Mutex<Client>,
    pub clusters: schema::Table<schema::clusters::K, schema::clusters::V>,
    pub definitions: schema::Table<schema::definitions::K, schema::definitions::V>
}

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data = web::Data::new(AppData {
        db: Mutex::new(Client::connect("", NoTls).unwrap()),
        clusters: schema::Table::new("clusters"),
        definitions: schema::Table::new("definitions")
    });

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .service(
                web::scope("/api")
                    .route("/cluster", web::post().to(api::cluster::post::post))
                    .route("/search", web::get().to(api::key_search::get_clusters_by_search_keys))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
