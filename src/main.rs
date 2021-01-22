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
    fn place_piece(&mut self, square: (H, V), p: Piece) -> Option<Board> {
        match get_piece(*self, square) {
            None => {
                self.pieces[h_to_i(square.0)][v_to_i(square.1)] = Some(p);
                Some(*self)
            }
            Some(_) => None
        }
    }
}

fn new_board() -> Board {
    Board {
        pieces: [[None; 4]; 4],
    }
}

#[derive(Copy, Clone)]
enum H {
    H1,
    H2,
    H3,
    H4,
}

#[derive(Copy, Clone)]
enum V {
    V1,
    V2,
    V3,
    V4
}

fn h_to_i(h: H) -> usize {
    match h {
        H::H1 => 1,
        H::H2 => 2,
        H::H3 => 3,
        H::H4 => 4,
    }
}

fn v_to_i(v: V) -> usize {
    match v {
        V::V1 => 1,
        V::V2 => 2,
        V::V3 => 3,
        V::V4 => 4,
    }
}

fn get_piece(b: Board, square: (H, V)) -> Option<Piece> {
    b.pieces[h_to_i(square.0)][v_to_i(square.1)]
}