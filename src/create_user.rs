use std::fs;
use crate::structs::User;
use crate::structs::UserData;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Responder};
use actix_web::http::header::LOCATION;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormData {
    username : String,
    password_hash : String
}

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body(include_str!("routes/create_user.html"))
}

pub async fn create_user_post(form: web::Form<FormData>) -> impl Responder {

    println!("{:?}", form);
    
    let new_user =  User {
        username: form.0.username,
        password_hash: form.0.password_hash,
    };

    //Read userlist json file, return error if username exists
    let userlist_json = fs::read_to_string("user_list.json")
        .expect("Something went wrong with the userlist");
    let mut users_vector: Vec<User> = serde_json::from_str(&userlist_json).unwrap();
    for every_user in &users_vector {
        if new_user.username == every_user.username {
            let msg = format!("Username already exists!");
            return HttpResponse::Ok().body(msg);
        }
    }
    //create json file for user and fill with empty dummy list
    let path = format!("userdata/{}.json", &new_user.username).trim().to_string();
    let path = std::path::Path::new(&path);
    std::fs::File::create(path)
        .expect("Could not create user file");
    let dummy =  UserData { lists : Vec::new()};
    let serialized = serde_json::to_string(&dummy).unwrap();
    fs::write(&path, &serialized).expect("Could not write to user file");

    //Add new user and write to userlist
    users_vector.push(new_user);
    let serialized = serde_json::to_string(&users_vector).unwrap();
    fs::write("user_list.json", &serialized).unwrap();
    
    
    //Call login
    println!("Account created. Login to proceed.\n");
    return HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish()
}