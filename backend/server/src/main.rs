use actix_web::{web, App, HttpResponse, HttpServer};
use chess_engine::board::Board;
use chess_engine::evaluator::Evaluator;
use chess_engine::move_validator::MoveValidator;
use chess_engine::piece::{Colour, PieceType};
use std::sync::Mutex;

struct AppState {
    board: Board,
    evaluator: Evaluator,
    move_validator: MoveValidator,
}

async fn index(data: web::Data<Mutex<AppState>>) -> HttpResponse {
    let mut state = data.lock().unwrap();
    state
        .board
        .place_piece(PieceType::Pawn, Colour::White, 0, 0);
    let result = state.board.get_value_at_square(0, 0);
    HttpResponse::Ok().body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Mutex::new(AppState {
        board: Board::new(),
        evaluator: Evaluator::new(),
        move_validator: MoveValidator::new(),
    }));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?;

    println!("Starting server on http://127.0.0.1:8080");
    webbrowser::open("http://127.0.0.1:8080").expect("Failed to open web browser.");
    server.run().await
}
