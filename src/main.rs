use quarto::*;
use structopt::StructOpt;

fn main() {
    // guards against passing arguments that won't be used.
    let _args = Cli::from_args();

    let b = new_board()
        .place_piece((Idx::I1, Idx::I2), WTRF)
        .unwrap_or_else(|| {new_board()})
        .place_piece((Idx::I2, Idx::I3), BTRF)
        .unwrap_or_else(|| {new_board()});

    println!("Board with two pieces:");
    println!("{}", b)
}

#[derive(StructOpt)]
struct Cli {
    // takes no arguments
}