#![allow(dead_code)]

use actix_web::{ web, App, HttpServer, Responder };
use postgres::{ Client, NoTls };
use std::sync::Mutex;

mod api;
mod error;
mod schema;

pub struct AppData {
    pub db: Mutex<Client>
}

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Mutex::new(Client::connect("host=localhost port=5432 user=postgres dbname=lang_notes password=example", NoTls).unwrap());
    schema::create_tables(&mut db.lock().unwrap()).unwrap();

    let data = web::Data::new(AppData {
        db
    });

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .app_data(data.clone())

                    .route("/cluster", web::post().to(api::cluster::post_cluster::post))
                    .route("/cluster", web::delete().to(api::cluster::delete_cluster::delete))

                    .route("/definition", web::post().to(api::definition::post_definition::post))
                    .route("/definition", web::get().to(api::definition::get_definition::get))
                    .route("/definition", web::put().to(api::definition::update_definition::update))
                    .route("/definition", web::delete().to(api::definition::delete_definition::delete))
                    
                    .route("/example", web::post().to(api::example::post_example::post))
                    .route("/example", web::get().to(api::example::get_example::get))
                    .route("/example", web::put().to(api::example::update_example::update))
                    .route("/example", web::delete().to(api::example::delete_example::delete))
                    
                    .route("/search", web::get().to(api::key_search::get_clusters_by_search_keys))

                    .route("/language", web::post().to(api::language::post_lang::post))
                    .route("/language", web::get().to(api::language::get_lang::get))
                    .route("/language", web::put().to(api::language::update_lang::update))
                    .route("/language", web::delete().to(api::language::delete_lang::delete))

                    .route("/translation", web::post().to(api::translation::post_translation::post))
                    .route("/translation", web::get().to(api::translation::get_translation::get))
                    .route("/translation", web::put().to(api::translation::update_translation::update))
                    .route("/translation", web::delete().to(api::translation::delete_translation::delete))

                    .route("/words_group", web::post().to(api::words_group::post_words_group::post))
                    .route("/words_group", web::get().to(api::words_group::get_words_group::get))
                    .route("/words_group", web::put().to(api::words_group::update_words_group::update))
                    .route("/words_group", web::delete().to(api::words_group::delete_words_group::delete))                    

            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
