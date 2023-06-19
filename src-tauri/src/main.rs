// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use chess::{Board, ChessMove, Color, Game, MoveGen, Piece, Square, PROMOTION_PIECES};

static mut GAME: Option<Game> = None;

fn main() {
    unsafe {
        GAME = Some(Game::new());

        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
                is_over,
                get_turn,
                do_a_move,
                get_possible_moves
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}

#[tauri::command]
unsafe fn is_over() -> bool {
    return match &GAME {
        Some(game) => game.result().is_some(),
        None => false,
    };
}

#[tauri::command]
unsafe fn get_turn() -> char {
    return match &GAME.clone().unwrap().side_to_move() {
        Color::White => 'w',
        Color::Black => 'b',
    };
}

#[tauri::command]
unsafe fn do_a_move(source: &str, destination: &str, promotion: &str) {
    //first check the possibility
    //then act

}

#[tauri::command]
unsafe fn get_possible_moves(square: &str) -> Vec<ChessMove> {

    let square = Square::from_str(square).unwrap();
    let mut possible_moves: Vec<ChessMove> = vec![];

    for possible_move in MoveGen::new_legal(&GAME.clone().unwrap().current_position()) {
        if possible_move.get_source() == square {
            possible_moves.push(possible_move);
        }
    }

    return possible_moves;
}
