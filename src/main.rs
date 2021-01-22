fn main() {
    println!("Hello, world!");
}

#[derive(Copy, Clone)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
enum Height {
    Tall,
    Short,
}

#[derive(Copy, Clone)]
enum Shape {
    Round,
    Square,
}

#[derive(Copy, Clone)]
enum Top {
    Flat,
    Hole,
}

#[derive(Copy, Clone)]
struct Piece {
    color: Color,
    height: Height,
    shape: Shape,
    top: Top,
}

#[derive(Copy, Clone)]
struct Board {
    pieces: [[Option<Piece>; 4]; 4],
}

impl Board {
    fn place_piece(&mut self, square: (Idx, Idx), p: Piece) -> Option<Board> {
        match self.get_piece(square) {
            None => {
                self.pieces[square.0.to_i()][square.1.to_i()] = Some(p);
                Some(*self)
            }
            Some(_) => None
        }
    }

    fn get_piece(&self, square: (Idx, Idx)) -> Option<Piece> {
        self.pieces[square.0.to_i()][square.1.to_i()]
    }
}

fn new_board() -> Board {
    Board {
        pieces: [[None; 4]; 4],
    }
}

#[derive(Copy, Clone)]
enum Idx {
    I1,
    I2,
    I3,
    I4,
}

impl Idx {
    fn to_i(&self) -> usize {
        match *self {
            Idx::I1 => 1,
            Idx::I2 => 2,
            Idx::I3 => 3,
            Idx::I4 => 4,
        }
    }
}