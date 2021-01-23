use either::Either;
use std::collections::HashMap;
use crate::board::*;
use crate::Idx::*;
use crate::piece::*;
use crate::piece::Attribute::*;

struct PassGame {
    board: Board,
}

struct PlaceGame {
    board: Board,
    passed: Piece
}

struct FinalGame {
    board: Board,
}

pub enum Game {
    Pass(PassGame),
    Place(PlaceGame),
    Final(FinalGame),
}

impl Game {
    fn board(&self) -> Board {
        match self {
            Game::Pass(g)  => g.board,
            Game::Place(g) => g.board,
            Game::Final(g) => g.board,
        }
    }
}

impl PassGame {
    pub fn pass_piece(self, p: Piece) -> Option<PlaceGame> {
        if self.board.contains(p) {
            None
        } else {
            Some(PlaceGame{ board: self.board, passed: p, })
        }
    }
}

impl PlaceGame {
    pub fn place_piece(self, square: (Idx, Idx)) -> Option<Either<FinalGame, PassGame>> {
        self.board.place_piece(square, self.passed).map(|b|
            if has_win(b) {
                Either::Left(FinalGame { board: b } )
            } else {
                Either::Right(PassGame { board: b } )
            }
        )
    }
}

fn row_has_win(row: [Option<Piece>; 4]) -> bool {
    fn r_has_win(r: [Piece; 4]) -> bool {
        let mut m = HashMap::new();

        let action: Vec<()> = r.iter().map(|p| {
            m.entry(C(p.color)).and_modify(|v| *v += 1).or_insert(1);
            m.entry(H(p.height)).and_modify(|v| *v += 1).or_insert(1);
            m.entry(S(p.shape)).and_modify(|v| *v += 1).or_insert(1);
            m.entry(T(p.top)).and_modify(|v| *v += 1).or_insert(1);
        }).collect();

        m.iter().any(|(_, &x)| x == 4)
    }

    match (row[0], row[1], row[2], row[3]) {
        (Some(a), Some(b), Some(c), Some(d)) => Some([a, b, c, d]),
        _ => None,
    }.iter().fold(false, |_, &r| r_has_win(r))
}

fn has_win(b: Board) -> bool {
    let mut found_win = false;
    let rows = b.raw();
    let cols = rows; // TODO rotate them first
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
        if row_has_win(*row) {
            found_win = true;
            break;
        }
    }

    found_win
}