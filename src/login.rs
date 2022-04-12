use std::fs;
use crate::structs::User;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Responder};
use actix_web::http::header::LOCATION;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormData {
    username : String,
    password_hash : String
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login(query: web::Query<QueryParams>) -> impl Responder {
    let error_html = match query.0.error {
        None => "".into(),
        Some(error_message) => format!("<p><i>{error_message}</i></p>"),
    };

    HttpResponse::Ok().body(error_html + include_str!("routes/login.html"))
}

pub async fn login_post(form: web::Form<FormData>) -> impl Responder {

    let new_login =  User {
        username: form.0.username,
        password_hash: form.0.password_hash,
    };

    //Read userlist and authenicate login, calling user_interface with login
    let userlist_json = fs::read_to_string("user_list.json")
        .expect("Something went wrong with the userlist");
    let users_vector: Vec<User> = serde_json::from_str(&userlist_json).unwrap();

    for every_user in &users_vector {
        if new_login.username == every_user.username && new_login.password_hash == every_user.password_hash {
            return HttpResponse::SeeOther()
            .insert_header((LOCATION, format!("/user_interface?username={}", new_login.username)))
            .finish()
        }
    }
    let msg = format!("Username, password do not match");
    return HttpResponse::SeeOther()
        .insert_header((LOCATION, format!("/login?error={}", msg)))
        .finish()

}