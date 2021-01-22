use std::fmt;

#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub enum Height {
    Tall,
    Short,
}

#[derive(Copy, Clone)]
pub enum Shape {
    Round,
    Square,
}

#[derive(Copy, Clone)]
pub enum Top {
    Flat,
    Hole,
}

#[derive(Copy, Clone)]
pub struct Piece {
    color: Color,
    height: Height,
    shape: Shape,
    top: Top,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_owned();
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