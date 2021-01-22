mod piece;
mod board;
mod game;
use crate::piece::Piece;
use crate::piece::Color;
use crate::piece::Height;
use crate::piece::Shape;
use crate::piece::Top;
use crate::board::Idx;
use crate::board::new_board;

fn main() {
    let wtrf = Piece {
        color: Color::White,
        height: Height::Tall,
        shape: Shape::Round,
        top: Top::Flat,
    };

    let b = new_board()
        .place_piece((Idx::I1, Idx::I2), wtrf)
        .unwrap_or_else(||{new_board()});

    println!("Board with one piece:");
    println!("{}", b)
}
