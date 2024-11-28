use crate::rules::othello as rules;
use rules::*;

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
/// let next_play = ai::predict(&board, Piece::Black, 0, 10).unwrap();
/// assert_eq!(next_play, Point::new(1, 1));
/// ```
pub fn predict(state: &Board, ai_player: Piece, rec: usize, width_lim: usize) -> Option<Point> {
    predict_rec(state, ai_player, rec, width_lim)
        .first()
        .map(|val| val.0)
}

/// returns vec sorted by score.
fn predict_rec(
    state: &Board,
    ai_player: Piece,
    rec: usize,
    width_lim: usize,
) -> Vec<(Point, usize)> {
    // MAX: state.size ^ 2
    let possible = state.placeable(ai_player);
    let current = state.score(ai_player);
    let mut possible: Vec<_> = possible
        .into_iter()
        .map(|point| (point, current + state.count_flips(point, ai_player)))
        .collect();
    possible.sort_by_key(|(_, score)| *score);
    possible.reverse();
    possible.truncate(width_lim);
    if rec == 0 {
        return possible;
    }
    let mut play_score_map = possible
        .into_iter()
        .map(|(ai_play, ai_score)| {
            let (next_board, placed) = state.clone().place(ai_play, ai_player).unwrap();
            assert_ne!(placed, 0);
            // this explodes unless you set width_lim to 1.
            let human_plays = predict_rec(&next_board, ai_player.flip(), rec - 1, 1);
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
        let next_play = predict(&board, Piece::Black, 0, 10).unwrap();
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
        let next_play = predict(&board, Piece::Black, 0, 10);
        assert_eq!(next_play, None);
    }
    #[test]
    fn recursion_should_terminate() {
        let board = "
            ........
            ........
            ........
            wbwwbbwb
            ...w.wbb
            ..wbbbw.
            .bbb....
            ..bb....
        ";
        let board = Board::decode(board, 8).unwrap();
        let next_play = predict(&board, Piece::Black, 10, 10);
        let next_board = board.place(next_play.unwrap(), Piece::Black).unwrap();
        drop(next_board);
    }
}
