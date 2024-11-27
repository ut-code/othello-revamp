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
    predict_rec(state, rec, ai_player).first().map(|val| val.0)
}

/// returns vec sorted by score.
fn predict_rec(state: &Board, rec: usize, ai_player: Piece) -> Vec<(Point, usize)> {
    let possible = state.placeable(ai_player);
    let current = state.score(ai_player);
    let mut possible: Vec<_> = possible
        .into_iter()
        .map(|point| (point, current + state.count_flips(point, ai_player)))
        .collect();
    possible.sort_by_key(|(_, score)| *score);
    possible.reverse();
    if rec == 0 {
        return possible;
    }
    // HACK: only calculating scores of TOP N to reduce calculation and mem use | N = rec + 1
    possible.truncate(rec + 1);
    let mut play_score_map = possible
        .into_iter()
        .map(|(ai_play, ai_score)| {
            let mut next_board = state.clone();
            assert_ne!(next_board.place(ai_play, ai_player).unwrap(), 0);
            let human_plays = predict_rec(&next_board, rec - 1, ai_player.flip());
            let human_score = human_plays
                .first()
                .map(|play| next_board.count_flips(play.0, ai_player.flip()))
                .unwrap_or(0);
            let flip_diff = ai_score - human_score; // larger is better for AI.
            (ai_play, flip_diff)
        })
        .collect::<Vec<_>>();
    play_score_map.sort_by_key(|play_score| play_score.1);
    play_score_map.reverse();
    play_score_map
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
    #[test]
    fn recursion_should_terminate() {
        let board = "
            ...w
            ..wb
            .bbb
            ..bb
        ";
        let board = Board::decode(board, 4).unwrap();
        let next_play = predict(&board, 10, Piece::Black);
        let mut next_board = board;
        next_board.place(next_play.unwrap(), Piece::Black).unwrap();
    }
}
