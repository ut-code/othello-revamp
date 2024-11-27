use std::fmt;

static EIGHT_DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub struct OutOfBoundaryError();
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
    pub fn move_for(&self, d: Direction) -> Result<Self, OutOfBoundaryError> {
        let res = Point {
            x: self.x.checked_add_signed(d.x).ok_or(OutOfBoundaryError())?,
            y: self.y.checked_add_signed(d.y).ok_or(OutOfBoundaryError())?,
        };
        Ok(res)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction {
    x: isize,
    y: isize,
}
impl Direction {
    pub fn times(&self, times: isize) -> Self {
        Self {
            x: self.x * times,
            y: self.y * times,
        }
    }
}
#[cfg(test)]
mod test_point {
    use super::*;
    #[test]
    fn oob_should_not_panic() {
        let dir = Direction { x: -1, y: -1 };
        let pt = Point { x: 0, y: 0 };
        pt.move_for(dir).ok();
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Black,
    White,
}
impl Piece {
    pub fn flip(&self) -> Self {
        match self {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
}
impl From<Piece> for Cell {
    fn from(val: Piece) -> Self {
        match val {
            Piece::Black => Cell::Black,
            Piece::White => Cell::White,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Black,
    White,
}
impl Cell {
    fn flip(&self) -> Cell {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
        }
    }
}
impl From<Cell> for Option<Piece> {
    fn from(val: Cell) -> Self {
        match val {
            Cell::Empty => None,
            Cell::Black => Some(Piece::Black),
            Cell::White => Some(Piece::White),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    size: usize,
    data: Vec<Vec<Cell>>,
}
impl Board {
    /// primitive opration. doesn't do anything other than setting the pice.
    /// returns Err iff at is out of boundary
    pub fn set(&mut self, at: Point, cell: impl Into<Cell>) -> Result<(), OutOfBoundaryError> {
        let row = self.data.get_mut(at.y).ok_or(OutOfBoundaryError())?;
        if row.len() <= at.x {
            return Err(OutOfBoundaryError());
        }
        row[at.x] = cell.into();
        Ok(())
    }
    pub fn get(&self, at: Point) -> Result<Cell, OutOfBoundaryError> {
        self.data
            .get(at.y)
            .ok_or(OutOfBoundaryError())?
            .get(at.x)
            .ok_or(OutOfBoundaryError())
            .copied()
    }
    pub fn new(size: usize) -> Self {
        assert!(size % 2 == 0, "size must be divisible by 2");
        assert!(size < 255, "size should not be larger than 255");
        let mut new = Self {
            size,
            data: vec![vec![Cell::Empty; size]; size],
        };
        new.set(Point::new(size / 2 - 1, size / 2 - 1), Piece::Black)
            .expect("this shouldn't happen");
        new.set(Point::new(size / 2 - 1, size / 2), Piece::White)
            .expect("this shouldn't happen");
        new.set(Point::new(size / 2, size / 2 - 1), Piece::White)
            .expect("this shouldn't happen");
        new.set(Point::new(size / 2, size / 2), Piece::Black)
            .expect("this shouldn't happen");
        new
    }
    /// flips pieces accordingly. returns Ok(count of flipped pieces) or Err(PlaceError).
    /// ```rust
    /// use boardgame_ai::rules::othello::*;
    /// let input = "
    ///     .wwb
    ///     ....
    ///     ....
    ///     ....
    /// ";
    /// let expected = "
    ///     bbbb
    ///     ....
    ///     ....
    ///     ....
    /// ";
    /// let mut board = Board::decode(input, 4).unwrap();
    /// let expected = Board::decode(expected, 4).unwrap();
    ///
    /// let flip_count = board.place(Point::new(0, 0), Piece::Black).unwrap();
    /// assert_eq!(board, expected);
    /// assert_eq!(flip_count, 2);
    /// ```
    pub fn place(&mut self, at: Point, piece: Piece) -> Result<usize, PlaceError> {
        let Ok(prev) = self.get(at) else {
            return Err(PlaceError::OutOfBoundary);
        };
        if prev != Cell::Empty {
            return Err(PlaceError::AlreadyOccupied);
        }
        self.set(at, piece).unwrap(); // shouldn't panic, as it would be already be return beforehand
        let mut count = 0;
        for dir in EIGHT_DIRECTIONS.iter().map(|&(x, y)| Direction { x, y }) {
            count += flip_in_direction(self, at, piece, dir);
        }
        if count == 0 {
            return Err(PlaceError::NoPiecesChanged);
        }
        Ok(count)
    }
    pub fn can_place(&self, at: Point, piece: Piece) -> bool {
        let Ok(prev) = self.get(at) else {
            return false; // can't place when it's out of the board
        };
        if prev != Cell::Empty {
            return false; // can't place when it's already occupied
        }
        for dir in EIGHT_DIRECTIONS.iter().map(|&(x, y)| Direction { x, y }) {
            if can_flip_in_direction(self, at, piece, dir) {
                return true;
            }
        }
        false // can't place when no pieces flip after placing
    }

    /// ```rust
    /// use boardgame_ai::rules::othello::Board;
    /// let serialized = "
    /// ......
    /// ..w...
    /// ..wb..
    /// .wwb..
    /// ...b..
    /// ......
    /// ";
    /// let board = Board::decode(serialized, 6).unwrap();
    /// assert_eq!(board.encode(), serialized);
    /// ```
    pub fn decode(serialized: &str, board_size: usize) -> Result<Board, DecodeError> {
        let rows: Vec<_> = serialized
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();
        if rows.len() != board_size {
            return Err(DecodeError::UnmatchedOverallLength {
                expected: board_size,
                got: rows.len(),
            });
        }
        let rows = rows
            .into_iter()
            .enumerate()
            .map(|(at, row)| {
                let row = row
                    .chars()
                    .map(|char| match char {
                        '.' | '_' => Ok(Cell::Empty), // _ can be used to emphasize cells
                        'w' => Ok(Cell::White),
                        'b' => Ok(Cell::Black),
                        _ => Err(DecodeError::UnknownChar(char)),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if row.len() == board_size {
                    Ok(row)
                } else {
                    Err(DecodeError::UnmatchedLocalLength {
                        at,
                        expected: board_size,
                        got: row.len(),
                    })
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(rows.len(), board_size);
        for row in rows.iter() {
            assert_eq!(row.len(), board_size);
        }
        Ok(Board {
            size: board_size,
            data: rows,
        })
    }
    pub fn encode(&self) -> String {
        "\n".to_string()
            + &self
                .data
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|cell| match cell {
                            Cell::Empty => ".",
                            Cell::Black => "b",
                            Cell::White => "w",
                        })
                        .collect::<Vec<&str>>()
                        .concat()
                })
                .collect::<Vec<String>>()
                .join("\n")
            + "\n"
    }
    pub fn score(&self, player: Piece) -> usize {
        let player_cell: Cell = player.into();
        self.data
            .iter()
            .map(|row| row.iter().filter(|&&cell| cell == player_cell).count())
            .sum()
    }

    /// ```rust
    /// use boardgame_ai::rules::othello::*;
    /// let serialized = "
    /// ......
    /// .bbb..
    /// ._w_..
    /// .bwb..
    /// .b_b..
    /// ......
    /// ";
    /// let board = Board::decode(serialized, 6).unwrap();
    /// let expected = vec![
    ///   Point::new(1, 2),
    ///   Point::new(2, 4),
    ///   Point::new(3, 2),
    /// ];
    /// assert_eq!(board.placable(Piece::Black), expected);
    /// ```
    pub fn placable(&self, next: Piece) -> Vec<Point> {
        // yes, this is O(n^2) in time, but does it really matter if the size of the board (=n) is less than 255
        // and it's Rust (not something slow and memory intensive like JS or Python)?
        let mut ret = Vec::new();
        for x in 0..self.size {
            for y in 0..self.size {
                let point = Point::new(x, y);
                if self.can_place(point, next) {
                    ret.push(point);
                }
            }
        }
        ret
    }
}
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = self.encode();
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod test_board {
    use super::*;
    #[test]
    fn decode() {
        let serialized = "
            ..b.bb
            ..bbbw
            b.wwb.
            wwbbb.
            ww.bbw
            ww.bbw
        ";
        let decoded = Board::decode(serialized, 6);
        assert!(decoded.is_ok(), "{:?}", decoded);
    }
    #[test]
    fn place_complex() {
        let input = "
            bbw.bb
            .wwbww
            bw.wb.
            wwbwb.
            ww.bww
            ww.bbb
        ";
        let expected = "
            bbw.bb
            .bwbww
            bbbbb.
            wwbbb.
            ww.bbw
            ww.bbb
        ";
        let mut board = Board::decode(input, 6).unwrap();
        let expected = Board::decode(expected, 6).unwrap();

        let flipped = board.place(Point::new(2, 2), Piece::Black).unwrap();

        assert_eq!(board, expected);
        assert_eq!(flipped, 5);
    }
    #[test]
    fn place_should_not_overwrite_past_stop() {
        let input = "
            .wwwb.
            .wwwbw
            .wwwbb
            .wwwwb
            ......
            ......
        ";
        let expected = "
            bbbbb.
            bbbbbw
            bbbbbb
            bbbbbb
            ......
            ......
        ";
        let mut board = Board::decode(input, 6).unwrap();
        let expected = Board::decode(expected, 6).unwrap();

        let mut flipped = board.place(Point::new(0, 0), Piece::Black).unwrap();
        flipped += board.place(Point::new(0, 1), Piece::Black).unwrap();
        flipped += board.place(Point::new(0, 2), Piece::Black).unwrap();
        flipped += board.place(Point::new(0, 3), Piece::Black).unwrap();

        assert_eq!(board, expected);
        assert_eq!(flipped, 13);
    }
    #[test]
    fn place_eight_directions() {
        let input = "
            b.b.b.
            .www..
            bw.wwb
            .www..
            b.w.w.
            ..b..b
        ";
        let expected = "
            b.b.b.
            .bbb..
            bbbbbb
            .bbb..
            b.b.b.
            ..b..b
        ";
        let mut board = Board::decode(input, 6).unwrap();
        let expected = Board::decode(expected, 6).unwrap();

        let flipped = board.place(Point::new(2, 2), Piece::Black).unwrap();

        assert_eq!(board, expected);
        assert_eq!(flipped, 11);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceError {
    AlreadyOccupied,
    OutOfBoundary,
    NoPiecesChanged,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    UnknownChar(char),
    UnmatchedOverallLength {
        expected: usize,
        got: usize,
    },
    UnmatchedLocalLength {
        at: usize,
        expected: usize,
        got: usize,
    },
}

// returns pieces that were flipped
pub fn flip_in_direction(b: &mut Board, at: Point, piece: Piece, direction: Direction) -> usize {
    if !can_flip_in_direction(b, at, piece, direction) {
        return 0;
    };
    let mut flipped = 0;
    loop {
        let Ok(pos) = at.move_for(direction.times(flipped as isize + 1)) else {
            break;
        };
        let Ok(cell) = b.get(pos) else {
            break;
        };
        if cell.flip() != piece.into() {
            break;
        }
        b.set(pos, piece).unwrap(); // it's safe to unwrap this
        flipped += 1;
    }
    flipped
}
// returns pieces that would be flipped, without flipping the pieces
pub fn can_flip_in_direction(b: &Board, at: Point, piece: Piece, direction: Direction) -> bool {
    let mut flipping_pieces: usize = 0;
    loop {
        let Ok(pos) = at.move_for(direction.times(flipping_pieces as isize + 1)) else {
            break false;
        };
        let Ok(cell) = b.get(pos) else {
            break false;
        };
        if cell == piece.into() && flipping_pieces > 0 {
            break true;
        }
        if cell != piece.flip().into() {
            break false;
        }
        flipping_pieces += 1;
    }
}
