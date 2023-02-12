/* This ia an actix Microservice that has multiple routes:
A. type: "/" that returns a message : "Hello, find a swimmer here!"
B. type: "/swimmer" that returns a random best swimmer in the list of the world top 5 best swimmers
C. type: "/version" that returns the version of the service
*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the random swimmer function from the lib.rs file
use jd_mini_proj4::random_swimmer;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, find a swimmer here!")
}

//create a function that returns a random best swimmer
#[get("/swimmer")]
async fn swimmer() -> impl Responder {
    //print the random movie
    println!("Random Swimmer: {}", random_swimmer());
    HttpResponse::Ok().body(random_swimmer())
}

//create a function that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    //print the version of the service
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(swimmer).service(version))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
