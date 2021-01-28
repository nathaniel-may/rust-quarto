use crate::piece::Piece;
use std::fmt;


#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub struct Board {
    // private field prevents ownership changes and therefor outside mutation
    pieces: [[Option<Piece>; 4]; 4],
}

impl Board {
    pub fn raw(&self) -> [[Option<Piece>; 4]; 4] {
        self.pieces.clone()
    }

    pub fn contains(&self, p: Piece) -> bool {
        let mut found = false;
        for row in &self.pieces {
            for piece in row {
                piece.map(|x|
                    if x == p { found = true; }
                );
            };
        };
        found
    }

    pub fn piece_count(&self) -> usize {
        self.pieces.iter().flatten().filter(|x| x.is_some()).count()
    }

    pub fn is_full(&self) -> bool {
        self.pieces.iter().flatten().all(|&x| x.is_some())
    }

    pub fn place_piece(&self, square: (Idx, Idx), p: Piece) -> Option<Board> {
        match self.get(square.0, square.1) {
            None => {
                let updated = &mut self.pieces.clone();
                updated[square.0.to_i()][square.1.to_i()] = Some(p);
                Some(Board { pieces: *updated })
            },
            Some(_) => None,
        }
    }

    pub fn get(&self, h: Idx, v: Idx) -> Option<Piece> {
        self.pieces[h.to_i()][v.to_i()]
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
            let mut row_s: String = "| ".to_owned();
            for p in row {
                match p {
                    None => row_s = row_s + "     | ",
                    Some(p) => row_s = row_s + &p.to_string() + " | ",
                };
            };
            row_s.pop(); // remove trailing space char
            s = s + &row_s + "\n";
        };
        write!(f, "{}", s)
    }
}

// TODO Eq
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Idx {
    I1,
    I2,
    I3,
    I4,
}

impl Idx {
    pub fn to_i(&self) -> usize {
        match *self {
            Idx::I1 => 0,
            Idx::I2 => 1,
            Idx::I3 => 2,
            Idx::I4 => 3,
        }
    }
}

pub static ALL_SQUARES: [(Idx, Idx); 16] = [
    (Idx::I1, Idx::I1),
    (Idx::I1, Idx::I2),
    (Idx::I1, Idx::I3),
    (Idx::I1, Idx::I4),
    (Idx::I2, Idx::I1),
    (Idx::I2, Idx::I2),
    (Idx::I2, Idx::I3),
    (Idx::I2, Idx::I4),
    (Idx::I3, Idx::I1),
    (Idx::I3, Idx::I2),
    (Idx::I3, Idx::I3),
    (Idx::I3, Idx::I4),
    (Idx::I4, Idx::I1),
    (Idx::I4, Idx::I2),
    (Idx::I4, Idx::I3),
    (Idx::I4, Idx::I4),
];