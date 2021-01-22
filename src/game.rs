use crate::board::Board;

pub struct Game {
    board: board,
    passed: Option<Piece>,
}

impl Game {
    pub fn place_piece(&mut self, square: (Idx, Idx), p: Piece) -> Option<Board> {
        match self.passed {
            Some(p') if p' == p => None,
            _ => self.board.place_piece(square, p),
        }
    }
}