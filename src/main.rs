use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use little_exif::u8conversion::U8conversion;
use serde::{Serialize, Deserialize};
use actix_files;
use std::path::Path;
mod file_reader;
mod tests;
mod trie;
use file_reader::*;
use regex::Regex;
use std::fs;
use little_exif::metadata::Metadata;
use little_exif::exif_tag::ExifTag;

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

async fn get_exif_data(file_path: web::Path<String>) -> Result<impl Responder, actix_web::Error> {
    let file_path = file_path.into_inner();

    let regex = Regex::new(r"^[a-zA-Z0-9_\-\s]+\.png$").unwrap();
    if !regex.is_match(&file_path) {
        return Err(actix_web::error::ErrorBadRequest("Invalid File Name!"));
    }

    let image_path = Path::new("./images").join(file_path);
    let metadata = Metadata::new_from_path(&image_path)?;
    let exif_description_tag = ExifTag::ImageDescription(String::new());

    if let Some(image_description) = metadata.get_tag(&exif_description_tag).next() {
        let endian = metadata.get_endian();
        let image_description_string = String::from_u8_vec(&image_description.value_as_u8_vec(&metadata.get_endian()), &endian);

        return Ok(HttpResponse::Ok().json(image_description_string));
    } else {
        return Ok(HttpResponse::Ok().json(""));
    }

}

/// Takes the file_path which the user inputted in the webpage, matches it
/// agaisnt the regex requiring it to have the extensions .png or .jpg, and
/// returns either the file path or an error
async fn serve_image(file_path: web::Path<String>) -> Result<impl Responder, actix_web::Error> {
    let path = file_path.into_inner();

    let regex = Regex::new(r"^[a-zA-Z0-9_\-\s]+\.png$").unwrap();
    if !regex.is_match(&path) {
        return Err(actix_web::error::ErrorBadRequest("Invalid File Name!"));
    }

    let file_path = Path::new("./images").join(path);

    let named_file = actix_files::NamedFile::open(file_path)
        .map_err(|_| actix_web::error::ErrorNotFound("File Not Found"))?;

    return Ok(named_file);
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    println!("Index File Served...");
    return actix_files::NamedFile::open(Path::new("static/index.html")).unwrap();
}

#[actix_web::get("/get_images")]
async fn get_images() -> impl Responder {
    let paths = fs::read_dir("./compressed_images").unwrap();
    let mut file_names: Vec<String> = Vec::new();

    for path in paths {
        match path {
            Ok(path) => {
                let file_name = path.file_name();
                let file_name: String = file_name.to_string_lossy().into_owned();
                file_names.push(file_name);
            } Err(err) => {
                eprintln!("Reason for Failure: {}", err.to_string());
                continue;
            }
        }
    }
    file_names.sort();
    return HttpResponse::Ok().json(file_names);
}

async fn serve_script(file_path: web::Path<String>) -> Result<impl Responder, actix_web::Error> {
    let path = file_path.into_inner();

    let regex = Regex::new(r"^[a-zA-Z0-9]+\-script\.js$").unwrap();
    if !regex.is_match(&path) {
        return Err(actix_web::error::ErrorBadRequest("Invalid File Name!"));
    }

    let file_path = Path::new("./scripts").join(path);

    let file = actix_files::NamedFile::open(file_path)
    .map_err(|_| actix_web::error::ErrorNotFound("File not Found"))?;

    return Ok(file);
}

async fn serve_compressed_image(file_path: web::Path<String>) -> Result<impl Responder, actix_web::Error> {
    let path = file_path.into_inner();

    let regex = Regex::new(r"^[a-zA-Z0-9_\-\s]+\.jpg$").unwrap();
    if !regex.is_match(&path) {
        return Err(actix_web::error::ErrorBadRequest("Invalid File Name!"));
    }

    let file_path = Path::new("./compressed_images").join(path);

    let named_file = actix_files::NamedFile::open(file_path)
        .map_err(|_| actix_web::error::ErrorNotFound("File Not Found"))?;

    return Ok(named_file);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
		    App::new()
	        .service(login)
	        .service(index)
            .route("/images/{file_name}", actix_web::web::get().to(get_exif_data))
            .route("/images/{file_name}", actix_web::web::get().to(serve_image))
            .service(get_images)
            .service(actix_files::Files::new("/photos", "./static").index_file("photos.html"))
            .route("/script/{file_name}", actix_web::web::get().to(serve_script))
            .route("/compressed/{file_name}", actix_web::web::get().to(serve_compressed_image))
    })
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
