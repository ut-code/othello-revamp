use crate::rules::othello as rules;
use rules::Board;
use rules::Piece;
use rules::Point;

pub fn eval(state: &Board, next_player: Piece) -> f64 {
    state.score(next_player) as f64
}
/// returns (best point to place, expected score).
/// the larger `rec` is, the better the AI plays. (and more resouce this program consumes)
/// will return None if there were no cells that AI can place.
/// ```rust
/// use boardgame_ai::ai::othello as ai;
/// use boardgame_ai::rules::othello::*;
/// let board = "
/// ....
/// ._wb
/// .wwb
/// .bbb
/// ";
/// let board = Board::decode(board, 4).unwrap();
/// let next_play = ai::predict(&board, 0, Piece::Black).unwrap();
/// assert_eq!(next_play, Point::new(1, 1));
/// ```
pub fn predict(state: &Board, rec: usize, ai_player: Piece) -> Option<Point> {
    predict_rec(state, rec, ai_player).first().copied()
}

fn predict_rec(state: &Board, rec: usize, ai_player: Piece) -> Vec<Point> {
    let mut possible = state.placeable(ai_player);
    // HACK: only calculating scores of TOP N to reduce calculation | N = rec + 1
    possible.sort_by_cached_key(|&point| state.count_flips(point, ai_player));
    possible.reverse();
    if rec == 0 {
        return possible;
    }
    let with_scores = possible
        .into_iter()
        .map(|ai_play| {
            let mut next_board = state.clone();
            next_board.place(ai_play, ai_player);
            let mut next_human_plays = predict_rec(&next_board, rec - 1, ai_player.flip());
            next_human_plays.truncate(rec - 1);
            next_human_plays
        })
        .collect::<Vec<_>>()
        .concat();
    todo!();
    with_scores
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn prediction() {
        let board = "
            bbbb
            wwww
            .wwb
            ....
        ";
        let board = Board::decode(board, 4).unwrap();
        let next_play = predict(&board, 0, Piece::Black).unwrap();
        assert_eq!(next_play, Point::new(0, 2));
    }
    #[test]
    fn when_ai_cannot_place() {
        let board = "
            ww.b
            bwbw
            w..b
            bwbw
        ";
        let board = Board::decode(board, 4).unwrap();
        let next_play = predict(&board, 0, Piece::Black);
        assert_eq!(next_play, None);
    }
}
