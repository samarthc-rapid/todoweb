use std::fs;
use crate::structs::UserData;
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Responder};
use actix_web::http::header::LOCATION;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormData {
    response : String,
    response_2 : String,
    list_name : String,
    task_name : String
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    username: String,
}

pub async fn user_interface(query: web::Query<QueryParams>) -> impl Responder {

     //load instance of Userdata from user's json file
    let user_filename = "userdata/".to_owned() + &query.0.username + ".json";
    let userdata_json = fs::read_to_string(&user_filename)
        .expect("Something went wrong with the userfile");
    let  userdata: UserData = serde_json::from_str(&userdata_json).unwrap();

    let mut result = include_str!("routes/user_interface.html").to_string(); 
    
    result.push_str(format!("Welcome {}, Your lists are : <br>", query.0.username).as_str());                                      //print names of lists
    for lists in &userdata.lists {
        result.push_str(format!("{}<br>", lists.0).as_str());
    }
    
    HttpResponse::Ok()
    .insert_header(("username", query.0.username.as_str()))
    .insert_header(ContentType::html())
    .body(result)

}


pub async fn user_interface_post(form: web::Form<FormData>, query: web::Query<QueryParams>) -> impl Responder{

    //load instance of Userdata from user's json file
    let user_filename = "userdata/".to_owned() + &query.0.username + ".json";
    let userdata_json = fs::read_to_string(&user_filename)
        .expect("Something went wrong with the userfile");
    let mut userdata: UserData = serde_json::from_str(&userdata_json).unwrap();

    let option = form.0.response.as_str();
    let list_name = form.0.list_name;

    let mut result : String;

        match option {
            "1" =>  {
                //Find a list name and print
                if let Some(pos) = userdata.lists.iter().position(|x| *x.0 == list_name) {
                    result = format!("List {}: \n\n {:?}", list_name,userdata.lists[pos]);
                }
                else {result = format!("No such list {}", list_name)}

            },
            "2" =>  {
                //Add new empty list
                let newlist =  (list_name.to_string(), Vec::new());
                userdata.lists.push(newlist);
                result = format!("List {} created successfully", list_name);
            },
            "3" => {
                //Find list
                if let Some(pos) = userdata.lists.iter().position(|x| *x.0 == list_name) {
                    let tasks = &mut userdata.lists[pos].1;
        
                
                    let option = form.0.response_2.as_str();
                    let task_name = form.0.task_name;

                    match option {
                        "1" => {
                            // Add new task by name
                            tasks.push((task_name, false));
                            result = format!("Task successfully added to list {}", list_name);
                        },
                        "2" => {
                            // Delete task by name
                            if let Some(pos) = tasks.iter().position(|x| *x.0 == task_name) {
                                tasks.remove(pos);
                                result = format!("Task {} deleted successfully", task_name);
                            }
                            else {result = format!("No such task {}", task_name);}
                        },
                        "3" => {
                            //Mark task done by name
                            if let Some(pos) = tasks.iter().position(|x| *x.0 == task_name) {
                                tasks[pos].1 = true;
                                result = format!("Task {} marked as done", task_name);
                            }
                            else {result = format!("No such task {}", task_name);}                           
                        },
                        _ => result = format!("Please try again"),
                    }
                }
                else {result = format!("No such list {}", list_name)}
                
            }
            "4" => {
                //Find list and delete
                if let Some(pos) = userdata.lists.iter().position(|x| *x.0 == list_name) {
                    userdata.lists.remove(pos);
                    result = format!("List {} deleted successfully", list_name);
                }
                else {result = format!("No such list {}", list_name)}
            }

            //logout to main
            "5" => {
                return HttpResponse::SeeOther()
                .insert_header((LOCATION, format!("/login")))
                .finish()
            }

            _ => result = format!("Please try again"),
        }
        //write any updates back to users json file
        let serialized = serde_json::to_string(&userdata).unwrap();
        fs::write(&user_filename, &serialized).expect("Could not write to user file");

        result.push_str(include_str!("routes/user_interface.html")); 
        result.push_str(format!("Welcome {}, Your lists are : <br>", query.0.username).as_str());                                      //print names of lists
        for lists in &userdata.lists {
            result.push_str(format!("{}<br>", lists.0).as_str());
        }
        HttpResponse::Ok()
        .insert_header(("username", query.0.username.as_str()))
        .insert_header(ContentType::html())
        .body(result)
}