use quarto::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use itertools::interleave;
use arrayvec::ArrayVec;
extern crate quickcheck;
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
use quickcheck::{Arbitrary, Gen};
use Turn::*;

static ALL_SQUARES: [(Idx, Idx); 16] = [
    (I1,I1),
    (I1,I2),
    (I1,I3),
    (I1,I4),
    (I2,I1),
    (I2,I2),
    (I2,I3),
    (I2,I4),
    (I3,I1),
    (I3,I2),
    (I3,I3),
    (I3,I4),
    (I4,I1),
    (I4,I2),
    (I4,I3),
    (I4,I4),
];

#[derive(Copy, Clone)]
#[derive(Debug)]
enum Turn {
    PassTurn(Piece),
    PlaceTurn((Idx, Idx)),
}

#[derive(Copy, Clone)]
#[derive(Debug)]
struct Run {
    turns: [Turn; 32],
}

impl Arbitrary for Run {
    fn arbitrary(g: &mut Gen) -> Run {
        let mut squares = ALL_SQUARES;
        let mut pieces = ALL_PIECES;

        let mut rng = thread_rng();
        squares.shuffle(&mut rng);
        pieces.shuffle(&mut rng);

        let interleaved: ArrayVec<[_; 32]> = 
            interleave(
                squares.iter().map(|x| PlaceTurn(*x)), 
                pieces.iter().map(|x| PassTurn(*x)))
            .into_iter()
            .collect();
        
        Run { turns: interleaved.into_inner().unwrap() }
    }
}

#[quickcheck]
fn all_games_end(r: Run) -> bool {
    false // TODO stub
}
