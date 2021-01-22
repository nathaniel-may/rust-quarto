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