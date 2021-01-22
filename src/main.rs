mod piece;
mod board;
use crate::board::new_board;

fn main() {
    println!("Empty Board:");
    println!("{}", new_board())
}
