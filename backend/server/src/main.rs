use actix_web::{web, App, HttpServer, HttpResponse};
use webbrowser;
use chess_engine::your_chess_engine_function;  // Import the necessary function(s)

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

// Handler for the "/hello" endpoint
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello!")
}

// Handler for calling chess function endpoint
async fn chess_function() -> HttpResponse {
    let result: String = your_chess_engine_function();
    HttpResponse::Ok().body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/chess").to(chess_function))
            .service(web::resource("/hello").to(hello))
    })
    .bind("127.0.0.1:8080")?;

    println!("Starting server on http://127.0.0.1:8080");

    // Open the default web browser
    webbrowser::open("http://127.0.0.1:8080").expect("Failed to open web browser.");

    server.run().await
}