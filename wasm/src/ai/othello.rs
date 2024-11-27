use crate::rules::othello as rules;
use rules::Board;
use rules::Piece;
use rules::Point;

pub fn eval(state: &Board, ai_player: Piece) -> f64 {
    state.score(ai_player) as f64
}
/// the larger `rec` is, the better the AI plays.
pub fn predict(state: &Board, rec: usize) -> Point {
    todo!()
}
