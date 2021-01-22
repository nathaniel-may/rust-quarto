use either::Either;
use crate::board::Board;
use crate::board::Idx;
use crate::piece::Piece;

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

    fn is_final(&self) -> bool {
        is_final(self.board())
    }
}

impl PassGame {
    pub fn pass_piece(self, p: Piece) -> Option<PlaceGame> {
        None // TODO stub
    }
}

impl PlaceGame {
    pub fn place_piece(self, square: (Idx, Idx)) -> Option<Either<FinalGame, PassGame>> {
        self.board.place_piece(square, self.passed).map(|b|
            if is_final(b) {
                Either::Left(FinalGame { board: b } )
            } else {
                Either::Right(PassGame { board: b } )
            }
        )
    }
}

fn is_final(b: Board) -> bool {
    false // TODO stub
}