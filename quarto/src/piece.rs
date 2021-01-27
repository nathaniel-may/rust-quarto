use std::fmt;

pub use Color::*;
pub use Height::*;
pub use Shape::*;
pub use Top::*;

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Height {
    Tall,
    Short,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Shape {
    Round,
    Square,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Top {
    Flat,
    Hole,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub struct Piece {
    pub color: Color,
    pub height: Height,
    pub shape: Shape,
    pub top: Top,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from("");
        match self.color {
            Color::White => s = s + "W",
            Color::Black => s = s + "B",
        }
        match self.height {
            Height::Tall => s = s + "T",
            Height::Short => s = s + "S",
        }
        match self.shape {
            Shape::Round => s = s + "R",
            Shape::Square => s = s + "Q",
        }
        match self.top {
            Top::Flat => s = s + "F",
            Top::Hole => s = s + "H",
        }
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Attribute {
    C(Color),
    H(Height),
    S(Shape),
    T(Top),
}

pub static WTRF: Piece = Piece { color: White, height: Tall, shape: Round, top: Flat };
pub static WTRH: Piece = Piece { color: White, height: Tall, shape: Round, top: Hole };
pub static WTQF: Piece = Piece { color: White, height: Tall, shape: Square, top: Flat };
pub static WTQH: Piece = Piece { color: White, height: Tall, shape: Square, top: Hole };
pub static WSRF: Piece = Piece { color: White, height: Short, shape: Round, top: Flat };
pub static WSRH: Piece = Piece { color: White, height: Short, shape: Round, top: Hole };
pub static WSQF: Piece = Piece { color: White, height: Short, shape: Square, top: Flat };
pub static WSQH: Piece = Piece { color: White, height: Short, shape: Square, top: Hole };
pub static BTRF: Piece = Piece { color: Black, height: Tall, shape: Round, top: Flat };
pub static BTRH: Piece = Piece { color: Black, height: Tall, shape: Round, top: Hole };
pub static BTQF: Piece = Piece { color: Black, height: Tall, shape: Square, top: Flat };
pub static BTQH: Piece = Piece { color: Black, height: Tall, shape: Square, top: Hole };
pub static BSRF: Piece = Piece { color: Black, height: Short, shape: Round, top: Flat };
pub static BSRH: Piece = Piece { color: Black, height: Short, shape: Round, top: Hole };
pub static BSQF: Piece = Piece { color: Black, height: Short, shape: Square, top: Flat };
pub static BSQH: Piece = Piece { color: Black, height: Short, shape: Square, top: Hole };

pub static ALL_PIECES: [Piece; 16] = [
    WTRF,
    WTRH,
    WTQF,
    WTQH,
    WSRF,
    WSRH,
    WSQF,
    WSQH,
    BTRF,
    BTRH,
    BTQF,
    BTQH,
    BSRF,
    BSRH,
    BSQF,
    BSQH,
];