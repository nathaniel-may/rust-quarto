use crate::piece::Piece;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Board {
    pieces: [[Option<Piece>; 4]; 4],
}

impl Board {
    fn place_piece(&mut self, square: (Idx, Idx), p: Piece) -> Option<Board> {
        match self.get_piece(square) {
            None => {
                self.pieces[square.0.to_i()][square.1.to_i()] = Some(p);
                Some(*self)
            }
            Some(_) => None
        }
    }

    fn get_piece(&self, square: (Idx, Idx)) -> Option<Piece> {
        self.pieces[square.0.to_i()][square.1.to_i()]
    }
}

pub fn new_board() -> Board {
    Board {
        pieces: [[None; 4]; 4],
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_owned();
        for row in &self.pieces {
            for p in row {
                match p {
                    None => s = s + "_ ",
                    Some(p) => s = s + &p.to_string() + " ",
                };
                s = s + "\n";
            };
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone)]
pub enum Idx {
    I1,
    I2,
    I3,
    I4,
}

impl Idx {
    fn to_i(&self) -> usize {
        match *self {
            Idx::I1 => 1,
            Idx::I2 => 2,
            Idx::I3 => 3,
            Idx::I4 => 4,
        }
    }
}