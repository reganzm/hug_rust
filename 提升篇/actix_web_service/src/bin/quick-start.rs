use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};
use std::io;
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug,Serialize,Deserialize)]
struct Hello{
    msg:String
}

#[actix_rt::main]
async fn main()-> io::Result<()>{
    HttpServer::new(move ||{
        App::new().service(web::resource("/").route(web::post().to(index)))
    })
    .bind("127.0.0.1:9999")?
    .run()
    .await
}

async fn index(item:web::Json<Hello>) -> HttpResponse {
    HttpResponse::Ok().json(&item.msg)
}