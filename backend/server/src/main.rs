use actix_web::{web, App, HttpResponse, HttpServer};
use chess_engine::board::Board;
use chess_engine::evaluator::Evaluator;
use chess_engine::move_validator::MoveValidator;

struct AppState {
    board: Board,
    evaluator: Evaluator,
    move_validator: MoveValidator,
}

async fn index(data: web::Data<AppState>) -> HttpResponse {
    let mut board = &data.board;
    HttpResponse::Ok().body("Hello, world!")
}

// Handler for the "/hello" endpoint
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        board: Board::new(),
        evaluator: Evaluator::new(),
        move_validator: MoveValidator::new(),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/").to(index))
            .service(web::resource("/hello").to(hello))
    })
    .bind("127.0.0.1:8080")?;

    println!("Starting server on http://127.0.0.1:8080");
    webbrowser::open("http://127.0.0.1:8080").expect("Failed to open web browser.");
    server.run().await
}
