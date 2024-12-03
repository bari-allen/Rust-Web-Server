use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
mod file_reader;
use file_reader::{create_new_user, file_exists, create_file};

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String
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

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
   return format!("Hello World!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
		    App::new()
		    .service(greet)
		    .service(create_user)
    })
	.bind(("127.0.0.1", 8080))?
	.workers(2)
	.run()
	.await
}
