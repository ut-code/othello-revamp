use std::fmt;

static EIGHT_DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (1, 0),
];
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}
#[derive(Debug, PartialEq, Eq)]
struct OutOfBoundaryError();
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
struct Direction {
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
enum Piece {
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
impl Into<Cell> for Piece {
    fn into(self) -> Cell {
        match self {
            Piece::Black => Cell::Black,
            Piece::White => Cell::White,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
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
impl Into<Option<Piece>> for Cell {
    fn into(self) -> Option<Piece> {
        match self {
            Cell::Empty => None,
            Cell::Black => Some(Piece::Black),
            Cell::White => Some(Piece::White),
        }
    }
}

// TODO: impl Debug
#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    rows: Vec<Vec<Cell>>,
}
impl Board {
    /// primitive opration. doesn't do anything other than setting the pice.
    pub fn set(&mut self, at: Point, cell: impl Into<Cell>) -> Result<(), OutOfBoundaryError> {
        let row = self.rows.get_mut(at.y).ok_or(OutOfBoundaryError())?;
        if row.len() <= at.x {
            return Err(OutOfBoundaryError());
        }
        row[at.x] = cell.into();
        Ok(())
    }
    pub fn get(&self, at: Point) -> Result<Cell, OutOfBoundaryError> {
        self.rows
            .get(at.y)
            .ok_or(OutOfBoundaryError())?
            .get(at.x)
            .ok_or(OutOfBoundaryError())
            .map(|val| *val)
    }
    pub fn new(rows: usize) -> Self {
        assert!(rows % 2 == 0, "rows must be divisible by 2");
        assert!(rows < 255, "rows should not be larger than 255");
        let rows = rows as usize;
        let mut new = Self {
            rows: vec![vec![Cell::Empty; rows]; rows],
        };
        new.set(Point::new(rows / 2 - 1, rows / 2 - 1), Piece::Black)
            .expect("this shouldn't happen");
        new.set(Point::new(rows / 2 - 1, rows / 2), Piece::White)
            .expect("this shouldn't happen");
        new.set(Point::new(rows / 2, rows / 2 - 1), Piece::White)
            .expect("this shouldn't happen");
        new.set(Point::new(rows / 2, rows / 2), Piece::Black)
            .expect("this shouldn't happen");
        new
    }
    /// flips pieces accordingly.
    pub fn place(&mut self, piece: Piece, at: Point) -> Result<(), PlaceError> {
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
        return Ok(());
    }

    /// ```rust
    /// use othello_wasm::othello::Board;
    /// let serialized = "
    /// ......
    /// ..w...
    /// ..wb..
    /// .wwb..
    /// ...b..
    /// ......
    /// ";
    /// assert!(Board::decode(serialized, 6).is_ok());
    /// ```
    pub fn decode(serialized: &str, board_size: usize) -> Result<Board, DecodeError> {
        let rows: Vec<_> = serialized
            .lines()
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
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
                        '.' => Ok(Cell::Empty),
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
        Ok(Board { rows })
    }
    pub fn encode(&self) -> String {
        "\n".to_string()
            + &self
                .rows
                .iter()
                .map(|row| {
                    row.into_iter()
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
    fn place() {
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
        let mut output = Board::decode(input, 6).unwrap();
        output.place(Piece::Black, Point::new(2, 2)).unwrap();
        let expected = Board::decode(expected, 6).unwrap();
        assert_eq!(output, expected);
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
    let mut i: usize = 0;
    loop {
        let Ok(pos) = at.move_for(direction.times(i as isize + 1)) else {
            break;
        };
        let Ok(cell) = b.get(pos) else {
            break;
        };
        if cell != piece.flip().into() {
            break;
        }
        b.set(pos, cell.flip()).unwrap();
        i += 1;
    }
    return i;
}
