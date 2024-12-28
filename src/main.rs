use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use actix_files;
use std::path::Path;
mod file_reader;
use file_reader::{create_new_user, file_exists, create_file, valid_user_input};


#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}

#[derive(Serialize)]
struct Response {
    message: String,
}

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

#[actix_web::post("/login")]
async fn login(data: web::Json<User>) -> impl Responder {
    let username: String = data.username.clone();
    let password: String = data.password.clone();
    let is_valid = valid_user_input(username.clone(), password);

    if is_valid.is_ok() {
	let response = Response {
	    message: format!("Login Successful! Welcome {}!", username),
	};
	 return HttpResponse::Ok().json(response);
    } else {
	let response = Response {
	    message: format!("Error: Could not login!"),
	};

	return HttpResponse::Unauthorized().json(response);
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
