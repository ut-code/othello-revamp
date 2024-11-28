use crate::rules::othello as rules;
use rules::*;

pub fn eval(state: Board, playing: Piece) -> isize {
    let base_score = state.score(playing) as isize;
    let size = state.size();
    let positional_score = state
        .cells()
        .into_iter()
        .map(|(point, cell)| {
            // mirrored and squashed to top left quarter for easier calc.
            let squashed = Point {
                x: if point.x > size / 2 {
                    size - point.x - 1
                } else {
                    point.x
                },
                y: if point.y > size / 2 {
                    size - point.y - 1
                } else {
                    point.y
                },
            };
            // asked chat gpt for the scores. I'm not familiar with othello nor AI.
            let score = if squashed.x == 0 && squashed.y == 0 {
                // corner
                10
            } else if squashed.x <= 1 && squashed.y <= 1 {
                // dangerous place around corner
                -4
            } else if squashed.x == 0 || squashed.y == 0 {
                // side of the board, more stable than middle
                2
            } else if squashed.x == 1 || squashed.y == 1 {
                // it's bad according to chat gpt?
                -1
            } else {
                0
            };
            (score, cell)
        })
        .map(|(mul, cell)| match cell {
            Cell::Empty => 0,
            _ if cell == playing.into() => mul,
            _ => -mul,
        })
        .sum::<isize>();

    positional_score + base_score
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
/// let next_play = ai::predict(&board, Piece::Black, 1, 10).unwrap();
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
) -> Vec<(Point, isize)> {
    assert!(
        rec <= 10,
        "rec should not be larger than 10, otherwise the order will explode"
    );
    let possible = state.placeable(ai_player);
    let mut possible: Vec<_> = possible
        .into_iter()
        .map(|play| {
            let next = state.clone().place(play, ai_player).unwrap().0;
            (play, eval(next, ai_player))
        })
        .collect();
    possible.sort_by_key(|(_, score)| *score);
    possible.reverse();
    possible.truncate(width_lim);
    if rec == 0 {
        return possible;
    }
    let mut play_score_map = possible
        .into_iter()
        .map(|(init_ai_play, _)| {
            // FIXME: this probably contains some logic duplication, but I'm not smart enough to fix it
            let (after_ai_board, placed) = state.clone().place(init_ai_play, ai_player).unwrap();
            assert_ne!(placed, 0);
            let human_plays = predict_rec(&after_ai_board, ai_player.flip(), 0, 1);
            let mut after_human = after_ai_board.clone();
            if let Some((play, _)) = human_plays.first() {
                after_human = after_human.place(*play, ai_player.flip()).unwrap().0;
            }
            let next_ai_board = predict_rec(&after_human, ai_player, rec - 1, 2)
                .first()
                .map(|play| after_human.clone().place(play.0, ai_player).unwrap().0)
                .unwrap_or(after_human);
            (init_ai_play, eval(next_ai_board, ai_player))
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
            ww.b
            ....
        ";
        let board = Board::decode(board, 4).unwrap();
        let next_play = predict(&board, Piece::Black, 0, 10).unwrap();
        assert_eq!(next_play, Point::new(0, 3));
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
            ............
            ............
            wbwwbbwb....
            ....wbwwbbwb
            ............
            ......bbbww.
            ............
            wbwwbbwb....
            ...w.wbb....
            ..wbbbw.....
            .bbb........
            ..bb........
        ";
        let board = Board::decode(board, 12).unwrap();
        let next_play = predict(&board, Piece::Black, 7, 10);
        // NOTE: it takes around 10x~ more time on test than on WAsm, because test runs on debug mode.
        // add --release flag to `cargo test` and it will magically be 10x faster.
        // (i.e. it's not a bug that wasm runs much faster than on native test, given same params)
        let next_board = board.place(next_play.unwrap(), Piece::Black).unwrap();
        drop(next_board);
    }
}
