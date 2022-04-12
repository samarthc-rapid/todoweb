mod structs;
mod login;
mod user_interface;
mod create_user;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::LOCATION;
use create_user::create_user;
use serde::{Deserialize, Serialize};


async fn home() -> impl Responder {
    HttpResponse::Ok().body(include_str!("routes/home.html"))
}

#[derive(Debug, Serialize, Deserialize)]
struct FormData {
    response : String
}
async fn home_post(form: web::Form<FormData>) -> HttpResponse {

    if form.0.response == "true" {
        HttpResponse::SeeOther()
        .insert_header((LOCATION, "/create_user"))
        .finish()
    }
    else {
        HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish()
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/", web::post().to(home_post))
            .route("/create_user", web::get().to(create_user))
            .route("/create_user", web::post().to(create_user::create_user_post))
            .route("/login", web::get().to(login::login))
            .route("/login", web::post().to(login::login_post))
            .route("/user_interface", web::get().to(user_interface::user_interface))
            .route("/user_interface", web::post().to(user_interface::user_interface_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}