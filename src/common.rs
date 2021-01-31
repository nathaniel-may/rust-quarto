use std::{
    io,
    time::Duration,
    thread,
};
use termion::{
    color, 
    style,
};

pub static BANNER: [&str; 6] = [
    "  ____                   _",
    " / __ \\                 | |",
    "| |  | |_   _  __ _ _ __| |_ ___",
    "| |  | | | | |/ _` | '__| __/ _ \\",
    "| |__| | |_| | (_| | |  | || (_) |",
    " \\___\\_\\\\__,_|\\__,_|_|   \\__\\___/"
];

pub fn write_banner_at<W: io::Write>(cursor: (u16, u16), f: &mut W) {
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

// used instead of the write! macro
pub fn write_at<W: io::Write>(pos: (u16, u16), f: &mut W, s: &str) {
    f.write_fmt(format_args!("{}{}", termion::cursor::Goto(pos.0, pos.1), s)).unwrap();
}

pub trait App {
    type State: Copy;
    type Action: Copy;
    type Output;

    /// initial state of the application
    fn initial_state() -> Self::State;

    /// write the state. // TODO allow errors?
    fn write_state<W: io::Write>(f: &mut W, state: Self::State);

    /// Define keypress behvior // TODO idle to option?
    fn action_from(key: Option<std::result::Result<termion::event::Key, std::io::Error>>) -> Self::Action;
    
    /// None = exit; Some holds the updated state
    fn step(state: Self::State, action: Self::Action) -> Option<Self::State>;

    /// None = continue; Some holds the desired result from the state
    fn output_from(state: Self::State) -> Option<Self::Output>;

    fn run<W: io::Write>(f: &mut W, input: &mut termion::input::Keys<termion::AsyncReader>, tick_ms: Duration) -> Option<Self::Output> {
        Self::run_from(Self::initial_state(), f, input, tick_ms)
    }

    fn run_from<W: io::Write>(initial_state: Self::State, f: &mut W, input: &mut termion::input::Keys<termion::AsyncReader>, tick_ms: Duration) -> Option<Self::Output> {
        let mut state = Some(initial_state);
        let mut action;
    
        // while state is `Some` and the output is not yet available
        while state.map(|s| Self::output_from(s).is_none()) == Some(true) {
            if let Some(s) = state {
                Self::write_state(f, s);
                f.flush().unwrap();
                action = Self::action_from(input.next());
                state = Self::step(s, action)
            }
            thread::sleep(tick_ms);
        }
    
        state.and_then(|s| Self::output_from(s))
    }
}