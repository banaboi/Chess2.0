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
    let board = &data.board;
    let result = board.get_value_at_square(0, 0);

    let print_result: String;
    if let Some(piece) = result.piece {
        print_result = format!(
            "Colour of the piece is {} and its type is {}",
            piece.get_colour(),
            piece.get_piece_type()
        );
    } else {
        print_result = String::from("There is no piece at this square");
    }
    HttpResponse::Ok().body(print_result)
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
    })
    .bind("127.0.0.1:8080")?;

    println!("Starting server on http://127.0.0.1:8080");
    webbrowser::open("http://127.0.0.1:8080").expect("Failed to open web browser.");
    server.run().await
}
