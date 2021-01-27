use quarto::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use itertools::interleave;
use arrayvec::ArrayVec;
use either::Either;
use quickcheck;
use quickcheck_macros::*;
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

impl Run {
    fn play(&self) -> Option<Game> {
        self.turns.iter().fold(
            Some(Pass(quarto::new_game())), 
            |game, &turn| game.and_then(|g| play(g, turn))
        )
    }
}

impl Arbitrary for Run {
    fn arbitrary(_: &mut Gen) -> Run {
        let mut squares = ALL_SQUARES;
        let mut pieces = ALL_PIECES;

        let mut rng = thread_rng();
        squares.shuffle(&mut rng);
        pieces.shuffle(&mut rng);

        let interleaved: ArrayVec<[_; 32]> = 
            interleave(
                pieces.iter().map(|x| PassTurn(*x)),
                squares.iter().map(|x| PlaceTurn(*x)))
            .into_iter()
            .collect();
        
        Run { turns: interleaved.into_inner().unwrap() }
    }
}

fn play(game: Game, turn: Turn) -> Option<Game> {
    match game {
        g @ Game::Final(_) => Some(g),
        Game::Pass(g) => match turn {
            PassTurn(p) => g.pass(p).map(|x| Place(x)),
            _ => None,
        },
        Game::Place(g) => match turn {
            PlaceTurn(p) => match g.place(p) {
                Some(Either::Left(next_g)) => Some(Final(next_g)),
                Some(Either::Right(next_g)) => Some(Pass(next_g)),
                _ => None,
            },
            _ => None,
        },
    }
}

#[test]
fn new_game_is_empty() {
    assert!(Pass(quarto::new_game()).piece_count() == 0)
}

#[quickcheck]
fn all_games_end(r: Run) -> bool {
    match r.play() {
        Some(Final(_)) => true,
        _ => false,
    }
}

#[quickcheck]
fn detects_ties_as_final(r: Run) -> bool {
    match r.play() {
        // all final games with all pieces placed without a win are ties
        Some(g@crate::Final(_)) => {
            if (g.piece_count() == 16) && !g.has_win() {
                g.is_tie()
            } else {
                // non ties don't fail this test
                true
            }
        },
        // all tie games must be final
        Some(g) => {
            let should_be_tie = (g.piece_count() == 16) && !g.has_win();
            !g.is_tie() && !should_be_tie
        },
        // no generated run should fail
        _ => false,
    }
}