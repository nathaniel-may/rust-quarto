use crate::common::*;

use std::io;
use termion::{
    color,
    event::Key
};

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Row {
    Top,
    Bottom,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub struct State {
    cursor: Row,
    mode: Option<Mode>,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Action {
    CursorUp,
    CursorDown,
    Submit,
    Quit,
    Idle,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Mode {
    PassAndPlay,
    LocalNetwork,
}

pub struct SplashApp {}

impl App for SplashApp {
    type State = State;
    type Action = Action;
    type Output = Mode;

    fn initial_state() -> Self::State {
        State { cursor: Row::Top, mode: None }
    }

    fn write_state<W: io::Write>(f: &mut W, state: Self::State) {
        let mut cursor: (u16, u16) = (1, 2);

        write_banner_at(cursor, f);
        cursor = (3, 8);

        cursor.1 += 1;
        if state.cursor == Row::Top {
            f.write_fmt(format_args!("{}{}         Pass and Play         {}",
                termion::cursor::Goto(cursor.0, cursor.1),
                color::Bg(color::Rgb(128, 128, 128)),
                color::Bg(color::Reset)
            )).unwrap();

            cursor.1 += 1;
            write_at(cursor, f, "         Local Network         ");
        } else {
            write_at(cursor, f, "         Pass and Play         ");

            cursor.1 += 1;
            f.write_fmt(format_args!("{}{}         Local Network         {}",
                termion::cursor::Goto(cursor.0, cursor.1),
                color::Bg(color::Rgb(128, 128, 128)),
                color::Bg(color::Reset)
            )).unwrap();
        }

        cursor.1 += 2;
        write_at(cursor, f, "         - q to quit -         ");
    }

    fn action_from(key: Option<std::result::Result<termion::event::Key, std::io::Error>>) -> Self::Action {
        match key {
            Some(Ok(key)) => match key {
                Key::Char('q')  => Action::Quit,
                Key::Char('\n') => Action::Submit,
                Key::Up         => Action::CursorUp,
                Key::Down       => Action::CursorDown,
                _               => Action::Idle,
            },
            // throws errors away
            _ => Action::Idle,
        }
    }
    
    fn step(state: Self::State, action: Self::Action) -> Option<Self::State> {
        match (action, state.cursor) {
            (Action::Quit, _) => None,
            (Action::Idle, _) => Some(state),
            (Action::Submit, Row::Top) => Some(State { cursor: state.cursor, mode: Some(Mode::PassAndPlay) }),
            (Action::Submit, Row::Bottom) => Some(State { cursor: state.cursor, mode: Some(Mode::LocalNetwork) }),
            (Action::CursorUp, _) => Some(State { cursor: Row::Top, mode: state.mode }),
            (Action::CursorDown, _) => Some(State { cursor: Row::Bottom, mode: state.mode }),
        }
    }

    fn output_from(state: Self::State) -> Option<Self::Output> {
        state.mode
    }
}
