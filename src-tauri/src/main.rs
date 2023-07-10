// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{str::FromStr, collections::HashMap, cmp::Ordering};

use chess::{Board, BoardStatus, ChessMove, Color, Game, MoveGen, Piece, Square};

const AI_WON: i16 = 1000;
const HUMAN_WON: i16 = -1000;
const DRAW: i16 = 0;

static mut GAME: Option<Game> = None;

fn main() {
    unsafe {
        GAME = Some(Game::new());

        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
                is_over,
                get_turn,
                do_a_move,
                get_possible_moves,
                fen,
                ai_move,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}

#[tauri::command]
unsafe fn ai_move() -> String {
    let mut new_game = GAME.clone().unwrap();

    // let mut cache: HashMap<u64, i16> = HashMap::new();
    new_game.make_move(
        minimax(&new_game.current_position(), true, 0, i16::MIN, i16::MAX)
            .0
            .unwrap(),
    );
    GAME = Some(new_game);

    return GAME.clone().unwrap().current_position().to_string();
}

fn minimax(
    board: &Board,
    is_maximizer: bool,
    depth: u8,
    mut alpha: i16,
    mut beta: i16,
    // cache: &mut HashMap<u64, i16>,
) -> (Option<ChessMove>, i16) {
    if board.status() == BoardStatus::Stalemate || board.status() == BoardStatus::Checkmate {
        let game: Game = Game::new_with_board(board.clone());

        return (
            None,
            match game.result().unwrap() {
                chess::GameResult::WhiteCheckmates => HUMAN_WON + depth as i16,
                chess::GameResult::WhiteResigns => AI_WON - depth as i16,
                chess::GameResult::BlackCheckmates => AI_WON - depth as i16,
                chess::GameResult::BlackResigns => HUMAN_WON + depth as i16,
                chess::GameResult::Stalemate => DRAW,
                chess::GameResult::DrawAccepted => DRAW,
                chess::GameResult::DrawDeclared => DRAW,
            },
        );
    }

    if depth == 8 {
        return (None, evaluate(board));
    }

    let mut best_move: Option<ChessMove> = None;
    let move_generator = MoveGen::new_legal(&board);
    if is_maximizer {
        let moves = move_generator.filter(|m| {
            return board.color_on(m.get_source()).unwrap() == Color::Black;
        });


        let mut sorted_moves = moves.collect::<Vec<ChessMove>>();
        sorted_moves.sort_by(|a, b| {
            if board.piece_on(a.get_dest()).is_none() && board.piece_on(b.get_dest()).is_none() {
                return Ordering::Equal;
            }
            else if board.piece_on(a.get_dest()).is_some() && board.piece_on(b.get_dest()).is_none() {
                return Ordering::Less;
            }
            else if board.piece_on(b.get_dest()).is_some() && board.piece_on(a.get_dest()).is_none() {
                return Ordering::Greater;
            }

            let mut a_score: i16 = 0;
            let mut b_score: i16 = 0;
            a_score += match board.piece_on(a.get_dest()).unwrap() {
                Piece::Pawn => 1,
                Piece::Knight => 3,
                Piece::Bishop => 3,
                Piece::Rook => 5,
                Piece::Queen => 9,
                Piece::King => 20,
            };
            b_score += match board.piece_on(b.get_dest()).unwrap() {
                Piece::Pawn => 1,
                Piece::Knight => 3,
                Piece::Bishop => 3,
                Piece::Rook => 5,
                Piece::Queen => 9,
                Piece::King => 20,
            };

            return a_score.cmp(&b_score);
        });

        for m in sorted_moves {
            let new_board = board.make_move_new(m);
            // let stuff: i16;
            // if cache.get(&new_board.get_hash()).is_some() {
            //     stuff = cache.get(&new_board.get_hash()).unwrap().to_owned();
            // } else {
            //     cache.insert(new_board.get_hash(), stuff);
            // }
                let stuff = minimax(&new_board, false, depth + 1, alpha, beta).1;

            if stuff > alpha {
                alpha = stuff;
                best_move = Some(m);
            }

            if alpha >= beta {
                return (Some(m), alpha - depth as i16);
            }
        }

        return (best_move, alpha - depth as i16);
    } else {
        let moves = move_generator.filter(|m| {
            return board.color_on(m.get_source()).unwrap() == Color::White;
        });
        
        let mut sorted_moves = moves.collect::<Vec<ChessMove>>();
        sorted_moves.sort_by(|a, b| {
            if board.piece_on(a.get_dest()).is_none() && board.piece_on(b.get_dest()).is_none() {
                return Ordering::Equal;
            }
            else if board.piece_on(a.get_dest()).is_some() && board.piece_on(b.get_dest()).is_none() {
                return Ordering::Less;
            }
            else if board.piece_on(b.get_dest()).is_some() && board.piece_on(a.get_dest()).is_none() {
                return Ordering::Greater;
            }

            let mut a_score: i16 = 0;
            let mut b_score: i16 = 0;
            a_score += match board.piece_on(a.get_dest()).unwrap() {
                Piece::Pawn => 1,
                Piece::Knight => 3,
                Piece::Bishop => 3,
                Piece::Rook => 5,
                Piece::Queen => 9,
                Piece::King => 20,
            };
            b_score += match board.piece_on(b.get_dest()).unwrap() {
                Piece::Pawn => 1,
                Piece::Knight => 3,
                Piece::Bishop => 3,
                Piece::Rook => 5,
                Piece::Queen => 9,
                Piece::King => 20,
            };

            return a_score.cmp(&b_score);
        });

        for m in sorted_moves {
            let new_board = board.make_move_new(m);
            // let stuff: i16;
            // if cache.get(&new_board.get_hash()).is_some() {
            //     stuff = cache.get(&new_board.get_hash()).unwrap().to_owned();
            // } else {
            //     cache.insert(new_board.get_hash(), stuff);
            // }
                let stuff = minimax(&new_board, true, depth + 1, alpha, beta).1;

            if stuff < beta {
                beta = stuff;
                best_move = Some(m);
            }

            if alpha >= beta {
                return (Some(m), beta + depth as i16);
            }
        }

        return (best_move, beta + depth as i16);
    }
}

fn evaluate(board: &Board) -> i16 {
    let mut score: i16 = 0;

    let white_pieces = board.color_combined(Color::White);

    white_pieces.for_each(|square| {
        score -= match board.piece_on(square).unwrap() {
            Piece::Pawn => 10,
            Piece::Knight => 30,
            Piece::Bishop => 30,
            Piece::Rook => 50,
            Piece::Queen => 90,
            Piece::King => 0,
        }
    });

    let black_pieces = board.color_combined(Color::Black);

    black_pieces.for_each(|square| {
        score += match board.piece_on(square).unwrap() {
            Piece::Pawn => 10,
            Piece::Knight => 30,
            Piece::Bishop => 30,
            Piece::Rook => 50,
            Piece::Queen => 90,
            Piece::King => 0,
        }
    });

    return score;
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
unsafe fn do_a_move(source: &str, destination: &str, promotion: &str) -> bool {
    let m = ChessMove::new(
        Square::from_str(source).unwrap(),
        Square::from_str(destination).unwrap(),
        None,
    );

    let mut new_game = GAME.clone().unwrap();

    let result = new_game.make_move(m);
    GAME = Some(new_game);

    return result;
}

#[tauri::command]
unsafe fn get_possible_moves(square: &str) -> Vec<String> {
    let square = Square::from_str(square).unwrap();
    let mut possible_moves: Vec<String> = vec![];

    for possible_move in MoveGen::new_legal(&GAME.clone().unwrap().current_position()) {
        if possible_move.get_source() == square {
            possible_moves.push(possible_move.get_dest().to_string());
        }
    }

    return possible_moves;
}

#[tauri::command]
unsafe fn fen() -> String {
    return GAME.clone().unwrap().current_position().to_string();
}
