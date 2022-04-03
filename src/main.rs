use std::{env, error::Error};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use serde::Serialize;

mod entities;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

#[get("/forums")]
async fn forums(client: web::Data<Client>) -> Result<impl Responder, Box<dyn Error>> {
    let collection: Collection<entities::forum::Forum> =
        client.database("mediavida").collection("forums");
    let mut cursor = collection.find(None, None).await?;

    let mut forums: Vec<entities::forum::Forum> = Vec::new();
    while let Some(forum) = cursor.try_next().await? {
        println!("title: {:?}", forum.title);
        let forum = entities::forum::Forum::new(forum.title, forum.link, forum.description);
        forums.push(forum);
    }
    Ok(HttpResponse::Ok().json(forums))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let client = Client::with_uri_str(client_uri)
        .await
        .expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(forums)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
