use std::io;
use termion::{
    color, style,
    event::Key
};

pub static BANNER: [&str; 6] = [
    "  ____                   _",
    " / __ \\                 | |",
    "| |  | |_   _  __ _ _ __| |_ ___",
    "| |  | | | | |/ _` | '__| __/ _ \\",
    "| |__| | |_| | (_| | |  | || (_) |",
    " \\___\\_\\\\__,_|\\__,_|_|   \\__\\___/"
];

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

//termion::input::Keys<termion::raw::RawTerminal<dyn io::Write>>
pub fn run<W: io::Write>(f: &mut W, input: &mut termion::input::Keys<termion::AsyncReader>) -> Option<Mode> {
    let mut state = Some(State { cursor: Row::Top, mode: None });
    let mut action;

    // while state is Some and state.mode is None
    while state.map(|s| s.mode.is_none()) == Some(true) {
        if let Some(s) = state {
            write_splash(f, s);
            f.flush().unwrap();
            action = action_from(input.next());
            state = step(s, action)
        }
    }

    // state.and_then(state.mode)
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

pub fn write_banner_at<W: io::Write>(f: &mut W, cursor: (u16, u16)) {
    for (idx, line) in BANNER.iter().enumerate() {
        let i = idx as u16 + cursor.0;
        f.write_fmt(format_args!(
            "{}{}{}{}{}{}",
            style::Bold,
            termion::cursor::Goto(cursor.1, i),
            color::Fg(color::Rgb(138, 43, 226)),
            line,
            color::Fg(color::Reset),
            style::Reset
        )).unwrap();
    }
}

fn write_splash<W: io::Write>(f: &mut W, state: State) {
    let mut cursor: (u16, u16) = (1, 2);

    write_banner_at(f, cursor);
    cursor = (3, 8);

    cursor.1 += 1;
    if state.cursor == Row::Top {
        f.write_fmt(format_args!("{}{}         Pass and Play         {}",
            termion::cursor::Goto(cursor.0, cursor.1),
            color::Bg(color::Rgb(128, 128, 128)),
            color::Bg(color::Reset)
        )).unwrap();

        cursor.1 += 1;
        f.write_fmt(format_args!("{}         Local Network         ",
            termion::cursor::Goto(cursor.0, cursor.1)
        )).unwrap();
    } else {
        f.write_fmt(format_args!("{}         Pass and Play         ",
            termion::cursor::Goto(cursor.0, cursor.1)
        )).unwrap();

        cursor.1 += 1;
        f.write_fmt(format_args!("{}{}         Local Network         {}",
            termion::cursor::Goto(cursor.0, cursor.1),
            color::Bg(color::Rgb(128, 128, 128)),
            color::Bg(color::Reset)
        )).unwrap();
    }

    cursor.1 += 1;
    // TODO write_at in common library
    f.write_fmt(format_args!("{}         - q to quit -         ",
        termion::cursor::Goto(cursor.0, cursor.1)
    )).unwrap();
}