mod board;
mod piece;

// re-exorts
pub use piece::{
    Piece,
    ALL_PIECES,
    WTRF, WTRH, WTQF, WTQH, WSRF, WSRH, WSQF, WSQH, BTRF, BTRH, BTQF, BTQH, BSRF, BSRH, BSQF, BSQH
};
pub use board::{
    Board, 
    Idx,
    Idx::{I1, I2, I3, I4},
    new_board
};
pub use self::Game::{Pass, Place, Final};

// local imports
use either::Either;
use std::collections::HashMap;
use piece::Attribute::*;

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub struct PassGame {
    board: Board,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub struct PlaceGame {
    board: Board,
    passed: Piece
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub struct FinalGame {
    board: Board,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Game {
    Pass(PassGame),
    Place(PlaceGame),
    Final(FinalGame),
}

impl Game {
    pub fn board(&self) -> &Board {
        match self {
            Game::Pass(g)  => &g.board,
            Game::Place(g) => &g.board,
            Game::Final(g) => &g.board,
        }
    }

    pub fn is_tie(&self) -> bool {
        match self {
            g @ Final(_) => g.board().is_full(),
            _ => false
        }
    }
    
    pub fn has_win(&self) -> bool {
        has_win(&self.board())
    }

    pub fn piece_count(&self) -> usize {
        self.board().piece_count()
    }
}

impl PassGame {
    pub fn pass(self, p: Piece) -> Option<PlaceGame> {
        if self.board.contains(p) {
            None
        } else {
            Some(PlaceGame{ board: self.board, passed: p, })
        }
    }
}

impl PlaceGame {
    pub fn place(self, square: (Idx, Idx)) -> Option<Either<FinalGame, PassGame>> {
        self.board.place_piece(square, self.passed).map(|b|
            if has_win(&b) || b.is_full() {
                Either::Left(FinalGame { board: b } )
            } else {
                Either::Right(PassGame { board: b } )
            }
        )
    }
}

pub fn new_game() -> PassGame {
    PassGame { board: board::new_board() }
}

fn row_has_win(row: &[Option<Piece>; 4]) -> bool {
    fn r_has_win(r: &[Piece; 4]) -> bool {
        let mut m = HashMap::new();

        for p in r.iter() {
            m.entry(C(p.color)).and_modify(|v| *v += 1).or_insert(1);
            m.entry(H(p.height)).and_modify(|v| *v += 1).or_insert(1);
            m.entry(S(p.shape)).and_modify(|v| *v += 1).or_insert(1);
            m.entry(T(p.top)).and_modify(|v| *v += 1).or_insert(1);
        };

        m.iter().any(|(_, &x)| x == 4)
    }

    match (row[0], row[1], row[2], row[3]) {
        (Some(a), Some(b), Some(c), Some(d)) => Some([a, b, c, d]),
        _ => None,
    }.iter().fold(false, |_, &r| r_has_win(&r))
}

// rotates 90 deg clockwise
fn rotate<T: Copy>(x: &[[T; 4]; 4]) -> [[T; 4]; 4] {
    [
        [x[3][0], x[2][0], x[1][0], x[0][0]],
        [x[3][1], x[2][1], x[1][1], x[0][1]],
        [x[3][2], x[2][2], x[1][2], x[0][2]],
        [x[3][3], x[2][3], x[1][3], x[0][3]],
    ]
}

fn has_win(b: &Board) -> bool {
    let mut found_win = false;
    let rows = b.raw();
    let cols = rotate(&rows);
    let diag1 = [
        b.get(I1, I1),
        b.get(I2, I2),
        b.get(I3, I3),
        b.get(I4, I4)
    ];
    let diag2 = [
        b.get(I1, I4),
        b.get(I2, I3),
        b.get(I3, I2),
        b.get(I4, I1)
    ];
    let win_lines: [[Option<Piece>; 4]; 10] = [
        rows[0],
        rows[1],
        rows[2],
        rows[3],
        cols[0],
        cols[1],
        cols[2],
        cols[3],
        diag1,
        diag2,
    ];

    for row in &win_lines {
        if row_has_win(row) {
            found_win = true;
            break;
        }
    }

    found_win
}