use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use actix_files;
use std::path::Path;
mod file_reader;
use file_reader::{create_new_user, file_exists, create_file, valid_user_input};

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

///Creates a new user given the inputted `Username` and `Password`
#[actix_web::post("/users")]
async fn create_user(user_data: web::Json<User>,  ) -> impl Responder {
    let username = user_data.username.clone();
    let password = user_data.password.clone();

    if !file_exists(){
	let _ = create_file();
    }
    let _ = create_new_user(username, password);

    return HttpResponse::Created().json(User {
	username: user_data.username.clone(),
	password: user_data.password.clone()
    });
}

///Logs the user in given the correct inputted `Username` and `Password`
///Calls to the valid_user_input function from file_reader to ensure
///the correct usernames and passwords are inputted
#[actix_web::post("/login")]
async fn login(data: web::Json<User>) -> impl Responder {
    let username: String = data.username.clone();
    let password: String = data.password.clone();
    let is_valid = valid_user_input(username.clone(), password);

    match is_valid {
        Ok((username, _password)) => {
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
    println!("The Server Is Starting at {:?}", std::env::current_dir().unwrap());

    HttpServer::new(|| {
		    App::new()
	    .service(login)
	    .service(index)
		
    })
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
