use crate::common::*;
use crate::splash;
use crate::local;
use crate::local::*;
use either::{
    Either, 
    Left, 
    Right,
};
use quarto::*;
use std::{
    cmp::min,
    io,
};
use termion::{
    color, 
    event::Key,
    style,
};

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub struct State {
    game: Game,
    player: Player,
    selection: Either<(bool, usize), (Idx, Idx)>,
    error: Option<&'static str>,
    conn: usize,
}

pub struct NetworkApp {}

impl App for NetworkApp {
    type State = State;
    type Action = local::Action;
    type Output = Self::State;

    // TODO tcp connection in state
    fn initial_state() -> Self::State {
        local::LocalApp::initial_state()
    }

    // TODO print waiting for opponent and all that.
    fn write_state<W: io::Write>(f: &mut W, state: Self::State) {
        local::LocalApp::write_state(f, state)
    }

    fn action_from(key: Option<std::result::Result<termion::event::Key, std::io::Error>>) -> Self::Action {
        local::LocalApp::action_from(key)
    }
    
    // block on network calls here
    fn step(state: Self::State, action: Self::Action) -> Option<Self::State> {
        match (action, state.game) {
            (Action::Quit, _) => None, // exits
            (_, Final(_)) => block_on(send(state)).and_then(|_| Some(state)),
            (Action::Idle, _) => Some(state), // do nothing
            (Action::Submit, _) => {
                let selection = match state.selection {
                    Left(cursor) => Left(ALL_PIECES[cursor.1 + if cursor.0 {0} else {8}]),
                    Right(square) => Right(square),
                };
                let new_cursor = match state.selection {
                    Left(_) => Right((I1, I1)),
                    Right(_) => Left((true, 0)),
                };
                match local::play(state.game, selection) {
                    None => Some(State { game: state.game, player: state.player, selection: state.selection, error: Some("try again.") }),
                    Some(g@Place(_)) => {
                        let s = State { game: g, player: state.player.switch(), selection: new_cursor, error: state.error };
                        block_on(send(s)).and_then(|_| Some(s));
                    },
                    Some(g) => {
                        let s = State { game: g, player: state.player, selection: new_cursor, error: state.error };
                        block_on(send(s)).and_then(|_| Some(s));
                    },
                }
            },
            (Action::Move(Direction::Up), Pass(_)) => match state.selection {
                Left(cursor) => Some(State { game: state.game, player: state.player, selection: Left((true, cursor.1)), error: None }),
                Right(_) => Some(State { game: state.game, player: state.player, selection: Left((true, 0)), error: None }), 
            },
            (Action::Move(Direction::Down), Pass(_)) => match state.selection {
                Left(cursor) => Some(State { game: state.game, player: state.player, selection: Left((false, cursor.1)), error: None }),
                Right(_) => Some(State { game: state.game, player: state.player, selection: Left((true, 0)), error: None }), 
            },
            (Action::Move(Direction::Left), Pass(_)) => match state.selection {
                Left(cursor) => Some(State { game: state.game, player: state.player, selection: Left((cursor.0, if cursor.1==0 {0} else {cursor.1-1})), error: None }),
                Right(_) => Some(State { game: state.game, player: state.player, selection: Left((true, 0)), error: None }), 
            },
            (Action::Move(Direction::Right), Pass(_)) => match state.selection {
                Left(cursor) => Some(State { game: state.game, player: state.player, selection: Left((cursor.0, min(7, cursor.1+1))), error: None }),
                Right(_) => Some(State { game: state.game, player: state.player, selection: Left((true, 0)), error: None }), 
            },
            (Action::Move(Direction::Up), Place(_)) => match state.selection {
                Left(_) => Some(State { game: state.game, player: state.player, selection: Right((I1, I1)), error: None }),
                Right(square) => Some(State { game: state.game, player: state.player, selection: Right((local::prev(square.0).unwrap_or(square.0), square.1)), error: None }), 
            },
            (Action::Move(Direction::Down), Place(_)) => match state.selection {
                Left(_) => Some(State { game: state.game, player: state.player, selection: Right((I1, I1)), error: None }),
                Right(square) => Some(State { game: state.game, player: state.player, selection: Right((local::next(square.0).unwrap_or(square.0), square.1)), error: None }), 
            },
            (Action::Move(Direction::Left), Place(_)) => match state.selection {
                Left(_) => Some(State { game: state.game, player: state.player, selection: Right((I1, I1)), error: None }),
                Right(square) => Some(State { game: state.game, player: state.player, selection: Right((square.0, local::prev(square.1).unwrap_or(square.1))), error: None }), 
            },
            (Action::Move(Direction::Right), Place(_)) => match state.selection {
                Left(_) => Some(State { game: state.game, player: state.player, selection: Right((I1, I1)), error: None }),
                Right(square) => Some(State { game: state.game, player: state.player, selection: Right((square.0, local::next(square.1).unwrap_or(square.1))), error: None }), 
            },
        }
    }

    fn output_from(state: Self::State) -> Option<Self::Output> {
        match state.game {
            Final(_) => Some(state),
            _        => None
        }
    }
}
