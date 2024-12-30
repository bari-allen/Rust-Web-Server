use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use actix_files;
use std::path::Path;
mod file_reader;
mod tests;
use file_reader::*;

///The struct for the data the user inputs
#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}

///The struct for our response
#[derive(Serialize)]
struct Response {
    message: String,
}

///Logs the user in given the correct inputted `Username` and `Password`
///Calls to the valid_user_input function from file_reader to ensure
///the correct usernames and passwords are inputted
#[actix_web::post("/login")]
async fn login(data: web::Json<User>) -> impl Responder {
    let username: String = data.username.clone();
    let password: String = data.password.clone();
    let is_valid = valid_user_input(&username, &password);

    match is_valid {
        Ok(_) => {
            let response = Response {
                message: format!("Login Successful! Welcome {}", username),
            };
            return HttpResponse::Ok().json(response);
        }
        Err(_error) => {
            let response = Response {
                message: format!("Login Failed: Invalid Username or Password!"),
            };
            return HttpResponse::Ok().json(response);
        }
    }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    println!("Index File Served...");
    return actix_files::NamedFile::open(Path::new("static/index.html")).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
		    App::new()
	    .service(login)
	    .service(index)
		
    })
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
