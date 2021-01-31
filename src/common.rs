use std::io;
use termion::{
    color, 
    style
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