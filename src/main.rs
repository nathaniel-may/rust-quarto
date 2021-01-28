use quarto::*;
use structopt::StructOpt;
use termion::screen::AlternateScreen;
use termion::{color, clear, style};
use std::io;
use std::thread;
use std::time::Duration;
use std::fmt::Display;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
use either::{Either, Left, Right};
use std::cmp::{min, max};
use termion::raw::RawTerminal;
use std::fmt;

static BANNER: [&str; 6] = [
    "  ____                   _",
    " / __ \\                 | |",
    "| |  | |_   _  __ _ _ __| |_ ___",
    "| |  | | | | |/ _` | '__| __/ _ \\",
    "| |__| | |_| | (_| | |  | || (_) |",
    " \\___\\_\\\\__,_|\\__,_|_|   \\__\\___/"
];

static ORIGIN: (u16, u16) = (1, 1);
static BOTTOM: (u16, u16) = (1, 6);

fn main() {
    // guards against passing arguments that won't be used.
    let _args = Cli::from_args();

    // initial state of the application to be rendered
    let initial_state = State {
        game: Pass(quarto::new_game()),
        selection: Left(0),
        error: None,
    };
    
    {
        let mut stdin = termion::async_stdin().keys();
        let mut stdout = stdout().into_raw_mode().unwrap();
    
        // prep the terminal
        write!(stdout, "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide).unwrap();

        write_banner(&mut stdout);

        write!(stdout, "{}q to exit. <Enter> to continue.",
            termion::cursor::Goto(3, 8)).unwrap();

        stdout.flush().unwrap();

        // wait for user
        let mut play = true;
        loop {
            let input = stdin.next();
            match input {
                None => {
                    thread::sleep(Duration::from_millis(50))
                },
                // throwing away errors
                Some(Err(_)) => {
                    thread::sleep(Duration::from_millis(50))
                },
                Some(Ok(key)) => match key {
                    Key::Char('q')  => { play = false; break },
                    Key::Char('\n') => { play = true; break },
                    _               => thread::sleep(Duration::from_millis(50)),
                },
            }
        }

        // run the app, or skip and go straight to the exit.
        if play {
            run(&mut stdout, &mut stdin, initial_state);
        }

        // show cursor, clear screen
        writeln!(stdout, "{}{}{}", 
            termion::cursor::Show,
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        ).unwrap();
    }
    println!("{}", "Done.")
}

// used instead of the write! macro
fn write_at<W: io::Write>(pos: (u16, u16), f: &mut W, s: &str) {
    f.write_fmt(format_args!("{}{}", termion::cursor::Goto(pos.0, pos.1), s)).unwrap();
    f.flush().unwrap();
}

fn clear<W: io::Write>(f: &mut W) {
    f.write_fmt(format_args!("{}", termion::clear::All)).unwrap();
    f.flush().unwrap();
}

fn write_banner<W: io::Write>(f: &mut W) {
    for (idx, line) in BANNER.iter().enumerate() {
        let i = (idx + 1) as u16;
        f.write_fmt(format_args!(
            "{}{}{}{}",
            termion::cursor::Goto(2, i),
            color::Fg(color::Rgb(138, 43, 226)),
            line,
            color::Fg(color::Reset)
        )).unwrap();
    }
}

fn write_piece<W: io::Write>(f: &mut W, op: &Option<Piece>, selected: bool) {
    let mut s: String = String::from("");
    match op {
        None => f.write_fmt(format_args!("{}","   ")).unwrap(),
        Some(p) => {
            match p.top {
                Top::Flat => s = s + " ",
                Top::Hole => s = s + "○",
            };
            match p.shape {
                Shape::Round => s = String::from("(") + &s + ")",
                Shape::Square => s = String::from("[") + &s + "]",
            };

            match (p.color, p.height) {
                (Color::White, Height::Tall) => 
                    f.write_fmt(format_args!("{c}{u}{piece}{reset_color}{reset_underline}",
                        c = color::Fg(color::Red),
                        u = style::Underline,
                        piece = s,
                        reset_color = color::Fg(color::Reset),
                        reset_underline = style::Reset
                    )).unwrap(),
                (Color::White, Height::Short) => 
                    f.write_fmt(format_args!("{c}{u}{piece}{reset_color}{reset_underline}",
                        c = color::Fg(color::Red),
                        u = style::NoUnderline,
                        piece = s,
                        reset_color = color::Fg(color::Reset),
                        reset_underline = style::Reset
                    )).unwrap(),
                (Color::Black, Height::Tall) => 
                    f.write_fmt(format_args!("{c}{u}{piece}{reset_color}{reset_underline}",
                        c = color::Fg(color::Blue),
                        u = style::Underline,
                        piece = s,
                        reset_color = color::Fg(color::Reset),
                        reset_underline = style::Reset
                    )).unwrap(),
                (Color::Black, Height::Short) => 
                    f.write_fmt(format_args!("{c}{u}{piece}{reset_color}{reset_underline}",
                        c = color::Fg(color::Blue),
                        u = style::NoUnderline,
                        piece = s,
                        reset_color = color::Fg(color::Reset),
                        reset_underline = style::Reset
                    )).unwrap(),
            }
        },
    }
}

fn write_state<W: io::Write>(f: &mut W, state: State)  {
    // clear all output
    f.write_fmt(format_args!("{}", termion::clear::All)).unwrap();

    write_banner(f);

    let mut cursor: (u16, u16) = (4, 8);

    // write game board out // TODO display passed piece, piece menu, and "cursor"
    for row in state.game.board().raw().iter() {
        f.write_fmt(format_args!("{}", termion::cursor::Goto(cursor.0, cursor.1))).unwrap();
        for p in row {
            f.write_fmt(format_args!("{}", "| ")).unwrap();
            write_piece(f, p, false);
            f.write_fmt(format_args!("{}", " ")).unwrap();
        };
        f.write_fmt(format_args!("{}", "|")).unwrap();
        cursor.1 += 1;
    };
    cursor.1 += 1;

    cursor.0 = 2;
    // write pass menu row 1
    f.write_fmt(format_args!("{}", termion::cursor::Goto(cursor.0, cursor.1))).unwrap();
    for p in &ALL_PIECES[..8] {
        write_piece(f, &Some(*p), false);
        f.write_fmt(format_args!(" ")).unwrap();
    }
    cursor.1 += 1;

    // write pass menu row 2
    f.write_fmt(format_args!("{}", termion::cursor::Goto(cursor.0, cursor.1))).unwrap();
    for p in &ALL_PIECES[8..] {
        write_piece(f, &Some(*p), false);
        f.write_fmt(format_args!(" ")).unwrap();
    }
    cursor.1 += 1;

    // write any error messages
    match state.error {
        None => {},
        Some(e) => {
            f.write_fmt(format_args!("{pos}{red}{err}{reset}",
                pos = termion::cursor::Goto(1, cursor.1),
                red = color::Fg(color::Red),
                err = e,
                reset = color::Fg(color::Reset)
            )).unwrap();
            cursor.1 += 1;
        },
    }
    
    f.flush().unwrap();
}

#[derive(StructOpt)]
struct Cli {
    // takes no arguments
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
struct State {
    game: Game,
    selection: Either<usize, (Idx, Idx)>,
    error: Option<&'static str>,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
enum Action {
    Quit,
    Submit,
    Move(Direction)
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn ask(stdin: &mut termion::input::Keys<termion::AsyncReader>) -> Action {
    let input = stdin.next();
    match input {
        None => {
            thread::sleep(Duration::from_millis(50));
            ask(stdin)
        },
        // throwing away errors
        Some(Err(_)) => {
            thread::sleep(Duration::from_millis(50));
            ask(stdin)
        },
        Some(Ok(key)) => match key {
            Key::Char('q')  => Action::Quit,
            Key::Char('\n') => Action::Submit,
            Key::Left       => Action::Move(Direction::Left),
            Key::Right      => Action::Move(Direction::Right),
            Key::Up         => Action::Move(Direction::Up),
            Key::Down       => Action::Move(Direction::Down),
            _               => ask(stdin), // TODO unrecognized key message
        },
    }
}

fn run<W: io::Write>(output: &mut W, input: &mut termion::input::Keys<termion::AsyncReader>, state: State) {
    write_state(output, state);
    let action = ask(input);
    match (action, state.game) {
        (Action::Quit, _) => { /* exits */ },
        (Action::Submit, _) => {
            let selection = match state.selection {
                Left(i) => Left(ALL_PIECES[i]),
                Right(square) => Right(square),
            };
            match play(state.game, selection) {
                None => run(output, input, State { game: state.game, selection: state.selection, error: Some("try again.") }),
                Some(g) => run(output, input,State { game: g, selection: state.selection, error: state.error }),
            }
        },
        (_, g@Final(_)) => {
            if g.is_tie() {
                write_at(BOTTOM, output, "tie game!!!")
            } else {
                write_at(BOTTOM, output, "you win!!!")
            }
            std::process::exit(1)
        },
        (Action::Move(Direction::Up), Pass(_)) => run(output, input, state),
        (Action::Move(Direction::Down), Pass(_)) => run(output, input, state),
        (Action::Move(Direction::Left), Pass(_)) => match state.selection {
            Left(i) => run(output, input, State { game: state.game, selection: Left(if i==0 {0} else {i-1}), error: None }),
            Right(_) => run(output, input, State { game: state.game, selection: Left(0), error: None }), 
        },
        (Action::Move(Direction::Right), Pass(_)) => match state.selection {
            Left(i) => run(output, input, State { game: state.game, selection: Left(min(16, i+1)), error: None }),
            Right(_) => run(output, input, State { game: state.game, selection: Left(0), error: None }), 
        },
        (Action::Move(_), Place(_)) => run(output, input, state), // TODO STUB
    }
}

fn play(game:Game, selection: Either<Piece, (Idx, Idx)>) -> Option<Game> {
    match (game, selection) {
        (Final(_), _) => None,
        (Pass(g), Left(p)) => g.pass(p).map(|x| Place(x)),
        (Pass(_), _) => None,
        (Place(g), Right(square)) => g.place(square).map(|y|
            merge(bimap(y, |x| Final(x), |x| Pass(x)))
        ),
        (Place(_), _) => None
    }
}

fn bimap<A, B, C, D>(z: Either<A, B>, f: fn(A) -> C, g: fn(B) -> D) -> Either<C, D> {
    match z {
        Left(x)  => Left(f(x)),
        Right(x) => Right(g(x)),
    }
}

fn merge<A>(z: Either<A, A>) -> A {
    match z {
        Left(x)  => x,
        Right(x) => x,
    }
}

fn next(i: Idx) -> Idx {
    match i {
        I1 => I2,
        I2 => I3,
        I3 => I4,
        I4 => I1,
    }
}

fn prev(i: Idx) -> Idx {
    match i {
        I1 => I4,
        I2 => I1,
        I3 => I2,
        I4 => I3,
    }
}