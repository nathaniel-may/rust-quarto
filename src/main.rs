use quarto::*;

fn main() {

    let b = new_board()
        .place_piece((Idx::I1, Idx::I2), WTRF)
        .unwrap_or_else(|| {new_board()})
        .place_piece((Idx::I2, Idx::I3), BTRF)
        .unwrap_or_else(|| {new_board()});

    println!("Board with two pieces:");
    println!("{}", b)
}
