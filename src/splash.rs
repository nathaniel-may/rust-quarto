use crate::common::*;

use std::{
    io, thread,
    time::Duration
};
use termion::{
    color,
    event::Key
};


#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
enum Row {
    Top,
    Bottom,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
struct State {
    cursor: Row,
    mode: Option<Mode>,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
enum Action {
    Switch,
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

// Entry point to splash screen
pub fn run<W: io::Write>(f: &mut W, input: &mut termion::input::Keys<termion::AsyncReader>, tick_ms: Duration) -> Option<Mode> {
    let mut state = Some(State { cursor: Row::Top, mode: None });
    let mut action;

    // while state is Some and state.mode is None
    while state.map(|s| s.mode.is_none()) == Some(true) {
        if let Some(s) = state {
            write_state(f, s);
            f.flush().unwrap();
            action = action_from(input.next());
            state = step(s, action)
        }
        thread::sleep(tick_ms);
    }

    state?.mode
}

fn step(state: State, action: Action) -> Option<State> {
    match (action, state.cursor) {
        (Action::Quit, _) => None,
        (Action::Idle, _) => Some(state),
        (Action::Submit, Row::Top) => Some(State { cursor: state.cursor, mode: Some(Mode::PassAndPlay) }),
        (Action::Submit, Row::Bottom) => Some(State { cursor: state.cursor, mode: Some(Mode::LocalNetwork) }),
        (Action::Switch, row) => Some(State { cursor: if row == Row::Top { Row::Bottom } else { Row::Top }, mode: state.mode }),
    }
}

fn action_from(key: Option<std::result::Result<termion::event::Key, std::io::Error>>) -> Action {
    match key {
        Some(Ok(key)) => match key {
            Key::Char('q')  => Action::Quit,
            Key::Char('\n') => Action::Submit,
            Key::Up         => Action::Switch,
            Key::Down       => Action::Switch,
            _               => Action::Idle,
        },
        // throws errors away
        _ => Action::Idle,
    }
}

fn write_state<W: io::Write>(f: &mut W, state: State) {
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