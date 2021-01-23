mod board;
mod piece;
// TODO main should only need to use the lib crate

use piece::Piece;
use piece::Color;
use piece::Height;
use piece::Shape;
use piece::Top;
use board::Idx;
use board::new_board;

fn main() {
    let wtrf = Piece {
        color: Color::White,
        height: Height::Tall,
        shape: Shape::Round,
        top: Top::Flat,
    };

    let btrf = Piece {
        color: Color::Black,
        height: Height::Tall,
        shape: Shape::Round,
        top: Top::Flat,
    };

    let b = new_board()
        .place_piece((Idx::I1, Idx::I2), wtrf)
        .unwrap_or_else(|| {new_board()})
        .place_piece((Idx::I2, Idx::I3), btrf)
        .unwrap_or_else(|| {new_board()});

    println!("Board with two pieces:");
    println!("{}", b)
}
