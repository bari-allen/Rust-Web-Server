use actix_web::{cookie::time::Date, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use actix_files;
use std::path::Path;
mod file_reader;
mod tests;
use file_reader::*;
use regex::Regex;
use std::fs;

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

/// Takes the file_path which the user inputted in the webpage, matches it
/// agaisnt the regex requiring it to have the extensions .png or .jpg, and
/// returns either the file path or an error
async fn serve_image(file_path: web::Path<String>) -> Result<impl Responder, actix_web::Error> {
    let path = file_path.into_inner();

    let regex = Regex::new(r"^[a-zA-Z0-9_\-\s]+\.(png|jpg)$").unwrap();
    if !regex.is_match(&path) {
        return Err(actix_web::error::ErrorBadRequest("Invalid File Name!"));
    }

    let file_path = format!("./images/{}", path);

    return Ok(actix_files::NamedFile::open(file_path)?);
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    println!("Index File Served...");
    return actix_files::NamedFile::open(Path::new("static/index.html")).unwrap();
}

#[actix_web::get("/get_images")]
async fn get_images() -> impl Responder {
    let paths = fs::read_dir("./images").unwrap();
    let mut file_names: Vec<std::ffi::OsString> = Vec::new();

    for path in paths {
        match path {
            Ok(path) => {
                let file_name = path.file_name();
                file_names.push(file_name);
            } Err(err) => {
                eprintln!("Reason for Failure: {}", err.to_string());
                continue;
            }
        }
    }

    return HttpResponse::Ok().json(file_names);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
		    App::new()
	    .service(login)
	    .service(index)
        .route("/images/{file_name}", actix_web::web::get().to(serve_image))
        .service(get_images)
    })
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
