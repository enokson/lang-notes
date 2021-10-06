use actix_web::{ web, App, HttpServer, Responder };
use postgres::{ Client };
use std::{
    sync::{ Mutex }
};

mod api;
mod schema;

pub struct AppData {
    pub db: Mutex<Client>
}

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/api")
                .service(
                    web::scope("/cluster")
                        .route("/", web::post().to(api::cluster::post::post))
                )
                // ...so this handles requests for `GET /app/index.html`
                .route("/search", web::get().to(api::key_search::get_clusters_by_search_keys)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
