use std::fmt;

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