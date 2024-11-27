use crate::rules::othello as rules;
use rules::Board;
use rules::Piece;
use rules::Point;

pub fn eval(state: &Board, next_player: Piece) -> f64 {
    state.score(next_player) as f64
}
/// the larger `rec` is, the better the AI plays. (and more resouce this program consumes)
/// will return None if there were no cells that AI can place.
/// ```rust
/// use boardgame_ai::ai::othello as ai;
/// use boardgame_ai::rules::othello as rules;
/// let board = "
/// ....
/// ._wb
/// .wwb
/// .bbb
/// ";
/// let board = rules::Board::decode(board, 4).unwrap();
/// let next_play = ai::predict(&board, 0, rules::Piece::Black).unwrap();
/// assert_eq!(next_play, rules::Point::new(1, 1));
/// ```
pub fn predict(state: &Board, rec: usize, ai_player: Piece) -> Option<Point> {
    let possible = state.placable(ai_player);
    let mut with_scores: Vec<_> = possible
        .into_iter()
        .map(|point| {
            let mut next = state.clone();
            next.place(point, ai_player).unwrap();
            let score = next.score(ai_player);
            drop(next);
            (point, score)
        })
        .collect();
    with_scores.sort_by_key(|(_, score)| *score);
    with_scores.reverse();
    if rec == 0 {
        return with_scores.first().map(|v| v.0);
    }
    todo!()
}
