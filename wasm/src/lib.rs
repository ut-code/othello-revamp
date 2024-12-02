pub mod ai;
pub mod rules;
mod utils;

use ai::othello::predict;
use rules::othello::{self as othello_rules, Board, Piece, Point};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm!");
}

#[wasm_bindgen]
pub fn version() -> String {
    "1.0.0".to_owned()
}

#[wasm_bindgen]
pub fn init_othello(size: usize) -> Board {
    othello_rules::Board::new(size)
}

#[wasm_bindgen]
pub fn placeable(board: &Board, player: Piece) -> usize {
    board.placeable(player).len()
}
#[wasm_bindgen]
pub fn can_place(board: &Board, at: &Point, player: Piece) -> bool {
    board.count_flips(*at, player) > 0
}
#[wasm_bindgen]
pub struct Scores {
    pub black: usize,
    pub white: usize,
}
#[wasm_bindgen]
pub fn count_score(board: &Board) -> Scores {
    Scores {
        black: board.score(Piece::Black),
        white: board.score(Piece::White),
    }
}

#[wasm_bindgen]
/// return either Ok(encoded board) or Err(error)
/// encoding specs:
///
/// - board must be encoded correctly
/// - player must be "w" or "b"
/// - at must be encoded as "{x},{y}"
///
/// fails if:
///
/// - the point is already occupied
/// - the point is not placeable place
pub fn place_at(board: &Board, player: Piece, at: &Point) -> Result<Board, String> {
    let board = board.clone().place(*at, player)?;
    Ok(board)
}

#[wasm_bindgen]
pub fn generate_ai_play(board: &Board, ai_player: Piece, strength: usize) -> Board {
    let next_play = predict(board, ai_player, strength, strength);
    match next_play {
        Some(play) => board.clone().place(play, ai_player).unwrap(),
        None => board.clone(),
    }
}
