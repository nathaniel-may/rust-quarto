use quarto::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
extern crate quickcheck;
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
use quickcheck::{Arbitrary, Gen};

fn all() -> [(Idx, Idx); 16] { 
    [
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
    ]
}

#[derive(Copy, Clone)]
#[derive(Debug)]
enum Run {
    Run([(Idx, Idx); 16])
}

impl Arbitrary for Run {
    fn arbitrary(g: &mut Gen) -> Run {
        let mut all = all();
        let mut rng = thread_rng();
        all.shuffle(&mut rng);
        Run::Run(all)
    }
}

#[quickcheck]
fn all_games_end(xs: Run) -> bool {
    false // TODO stub
}