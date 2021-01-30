use quarto::*;
use structopt::StructOpt;
use termion::{color, style};
use std::io;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
use either::{Either, Left, Right};
use std::cmp::min;

static BANNER: [&str; 6] = [
    "  ____                   _",
    " / __ \\                 | |",
    "| |  | |_   _  __ _ _ __| |_ ___",
    "| |  | | | | |/ _` | '__| __/ _ \\",
    "| |__| | |_| | (_| | |  | || (_) |",
    " \\___\\_\\\\__,_|\\__,_|_|   \\__\\___/"
];

static TICK_MS: Duration = Duration::from_millis(50); 

fn main() {
    // guards against passing arguments that won't be used.
    let _args = Cli::from_args();
    
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

        // initial state of the application to be rendered
        let initial_state = State {
            game: quarto::new_game().to_game(),
            selection: Left((true, 0)),
            error: None,
        };

        // wait for user to read splash screen
        let mut action = wait_for_user(
            &mut stdin, 
            |x| match x {
                Key::Char('q')  => Some(Action::Quit),
                Key::Char('\n') => Some(Action::Idle),
                _               => None,
            }
        );

        // variable that starts the same. `None` exits the game instead.
        let mut state = Some(initial_state)
            .and_then(|s| step(s, action));

        // run the app event loop
        while state.is_some() {
            if let Some(s) = state {
                write_state(&mut stdout, s);
                stdout.flush().unwrap();
                if s.game.is_final() {
                    action = wait_for_user(
                        &mut stdin, 
                        |_| Some(Action::Quit)
                    );
                } else {
                    action = action_from(stdin.next());
                }
                state = step(s, action);
                thread::sleep(TICK_MS);
            }
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
}

fn write_banner<W: io::Write>(f: &mut W) {
    for (idx, line) in BANNER.iter().enumerate() {
        let i = (idx + 1) as u16;
        f.write_fmt(format_args!(
            "{}{}{}{}{}{}",
            style::Bold,
            termion::cursor::Goto(2, i),
            color::Fg(color::Rgb(138, 43, 226)),
            line,
            color::Fg(color::Reset),
            style::Reset
        )).unwrap();
    }
}

fn write_piece<W: io::Write>(f: &mut W, op: &Option<Piece>, selected: bool) {
    match op {
        None => {
            if selected {
                f.write_fmt(format_args!("{bg}{empty}{reset_bg}",
                    bg = color::Bg(color::Rgb(128,128,128)),
                    empty = "   ",
                    reset_bg = color::Bg(color::Reset)
                )).unwrap()
            } else {
                f.write_fmt(format_args!("{}","   ")).unwrap()
            }
        },
        Some(p) => {
            let mut s: String = String::from("");
            match p.top {
                Top::Flat => s = s + " ",
                Top::Hole => s = s + "○",
            };
            match p.shape {
                Shape::Round => s = String::from("(") + &s + ")",
                Shape::Square => s = String::from("[") + &s + "]",
            };

            let color = match p.color {
                Color::White => color::Fg(color::Rgb(255, 0, 0)),
                Color::Black => color::Fg(color::Rgb(0, 64, 255)),
            };

            // directly match poorly designed termion types
            match (p.height, selected) {
                (Height::Tall, true)  => {
                    f.write_fmt(format_args!("{bold}{c}{bg}{u}{piece}{reset_color}{reset_bg}{reset_style}",
                        bold = style::Bold,
                        c = color,
                        bg = color::Bg(color::Rgb(128,128,128)),
                        u = style::Underline,
                        piece = s,
                        reset_color = color::Fg(color::Reset),
                        reset_bg = color::Bg(color::Reset),
                        reset_style = style::Reset
                    )).unwrap()
                },
                (Height::Short, true) => {
                    f.write_fmt(format_args!("{bold}{c}{bg}{u}{piece}{reset_color}{reset_bg}{reset_style}",
                        bold = style::Bold,    
                        c = color,
                        bg = color::Bg(color::Rgb(128,128,128)),
                        u = style::NoUnderline,
                        piece = s,
                        reset_color = color::Fg(color::Reset),
                        reset_bg = color::Bg(color::Reset),
                        reset_style = style::Reset
                    )).unwrap()
                },
                (Height::Tall, false)  => {
                    f.write_fmt(format_args!("{bold}{c}{bg}{u}{piece}{reset_color}{reset_bg}{reset_style}",
                        bold = style::Bold,
                        c = color,
                        bg = color::Bg(color::Reset),
                        u = style::Underline,
                        piece = s,
                        reset_color = color::Fg(color::Reset),
                        reset_bg = color::Bg(color::Reset),
                        reset_style = style::Reset
                    )).unwrap()
                },
                (Height::Short, false) => {
                    f.write_fmt(format_args!("{bold}{c}{bg}{u}{piece}{reset_color}{reset_bg}{reset_style}",
                        bold = style::Bold,
                        c = color,
                        bg = color::Bg(color::Reset),
                        u = style::NoUnderline,
                        piece = s,
                        reset_color = color::Fg(color::Reset),
                        reset_bg = color::Bg(color::Reset),
                        reset_style = style::Reset
                    )).unwrap()
                },
            };
        },
    }
}

fn write_state<W: io::Write>(f: &mut W, state: State)  {
    // clear all output
    f.write_fmt(format_args!("{}", termion::clear::All)).unwrap();

    write_banner(f);

    let mut cursor: (u16, u16) = (6, 8);

    // write game board out
    let mut square = (I1, I1);
    for row in state.game.board().raw().iter() {
        f.write_fmt(format_args!("{}", termion::cursor::Goto(cursor.0, cursor.1))).unwrap();
        for p in row {
            let passed_or_placed = 
                if either::Right(square) == state.selection && p.is_none() {
                    state.game.get_passed_piece()
                } else  {
                    *p
                };
            f.write_fmt(format_args!("{}", "| ")).unwrap();
            write_piece(f, &passed_or_placed, either::Right(square) == state.selection);
            f.write_fmt(format_args!("{}", " ")).unwrap();
            square.1 = next(square.1).unwrap_or(I1);
        };
        square.0 = next(square.0).unwrap_or(square.0);
        f.write_fmt(format_args!("{}", "|")).unwrap();
        cursor.1 += 1;
    };
    cursor.1 += 1;

    cursor.0 = 3;
    // write pass menu row 1
    let mut piece_cursor = (true, 0);
    f.write_fmt(format_args!("{}", termion::cursor::Goto(cursor.0, cursor.1))).unwrap();
    for p in &ALL_PIECES[..8] {
        let available_piece = if state.game.contains(p) { None } else { Some(*p) };
        write_piece(f, &available_piece, either::Left(piece_cursor) == state.selection);
        f.write_fmt(format_args!(" ")).unwrap();
        piece_cursor.1 += 1;
    }
    cursor.1 += 1;

    // write pass menu row 2
    piece_cursor = (false, 0);
    f.write_fmt(format_args!("{}", termion::cursor::Goto(cursor.0, cursor.1))).unwrap();
    for p in &ALL_PIECES[8..] {
        let available_piece = if state.game.contains(p) { None } else { Some(*p) };
        write_piece(f, &available_piece, either::Left(piece_cursor) == state.selection);
        f.write_fmt(format_args!(" ")).unwrap();
        piece_cursor.1 += 1;
    }
    cursor.1 += 2;

    // write any error messages
    cursor.0 = 14;
    match state.error {
        None => {},
        Some(e) => {
            f.write_fmt(format_args!("{pos}{red}{err}{reset}",
                pos = termion::cursor::Goto(cursor.0, cursor.1),
                red = color::Fg(color::Red),
                err = e,
                reset = color::Fg(color::Reset)
            )).unwrap();
            cursor.1 += 2;
        },
    }

    match state.game {
        g@Final(_) => {
            if g.is_tie() {
                write_at(cursor, f, "tie game!!!")
            } else {
                write_at(cursor, f, "you win!!!")
            };
            cursor.1 += 1;
            cursor.0 = 8;
            write_at(cursor, f, "press any key to exit.");
        },
        _ => {},
    }
}

fn wait_for_user(input: &mut termion::input::Keys<termion::AsyncReader>, f: fn(termion::event::Key) -> Option<Action>) -> Action {
    match input.next() {
        Some(Ok(key)) => match f(key) {
            None => wait_for_user(input, f),
            Some(action) => action,
        },
        // throwing away errors
        _ => {
            thread::sleep(TICK_MS);
            wait_for_user(input, f)
        },
    }
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
    selection: Either<(bool, usize), (Idx, Idx)>,
    error: Option<&'static str>,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
enum Action {
    Quit,
    Submit,
    Move(Direction), 
    Idle,
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

fn action_from(key: Option<std::result::Result<termion::event::Key, std::io::Error>>) -> Action {
    match key {
        Some(Ok(key)) => match key {
            Key::Char('q')  => Action::Quit,
            Key::Char('\n') => Action::Submit,
            Key::Left       => Action::Move(Direction::Left),
            Key::Right      => Action::Move(Direction::Right),
            Key::Up         => Action::Move(Direction::Up),
            Key::Down       => Action::Move(Direction::Down),
            _               => Action::Idle,
        },
        // throws errors away
        _ => Action::Idle,
    }
}

fn step(state: State, action: Action) -> Option<State> {
    match (action, state.game) {
        (Action::Quit, _) => None, // exits
        (_, Final(_)) => Some(state), // do nothing. exit controlled in event loop.
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
            match play(state.game, selection) {
                None => Some(State { game: state.game, selection: state.selection, error: Some("try again.") }),
                Some(g) => Some(State { game: g, selection: new_cursor, error: state.error }),
            }
        },
        (Action::Move(Direction::Up), Pass(_)) => match state.selection {
            Left(cursor) => Some(State { game: state.game, selection: Left((true, cursor.1)), error: None }),
            Right(_) => Some(State { game: state.game, selection: Left((true, 0)), error: None }), 
        },
        (Action::Move(Direction::Down), Pass(_)) => match state.selection {
            Left(cursor) => Some(State { game: state.game, selection: Left((false, cursor.1)), error: None }),
            Right(_) => Some(State { game: state.game, selection: Left((true, 0)), error: None }), 
        },
        (Action::Move(Direction::Left), Pass(_)) => match state.selection {
            Left(cursor) => Some(State { game: state.game, selection: Left((cursor.0, if cursor.1==0 {0} else {cursor.1-1})), error: None }),
            Right(_) => Some(State { game: state.game, selection: Left((true, 0)), error: None }), 
        },
        (Action::Move(Direction::Right), Pass(_)) => match state.selection {
            Left(cursor) => Some(State { game: state.game, selection: Left((cursor.0, min(7, cursor.1+1))), error: None }),
            Right(_) => Some(State { game: state.game, selection: Left((true, 0)), error: None }), 
        },
        (Action::Move(Direction::Up), Place(_)) => match state.selection {
            Left(_) => Some(State { game: state.game, selection: Right((I1, I1)), error: None }),
            Right(square) => Some(State { game: state.game, selection: Right((prev(square.0).unwrap_or(square.0), square.1)), error: None }), 
        },
        (Action::Move(Direction::Down), Place(_)) => match state.selection {
            Left(_) => Some(State { game: state.game, selection: Right((I1, I1)), error: None }),
            Right(square) => Some(State { game: state.game, selection: Right((next(square.0).unwrap_or(square.0), square.1)), error: None }), 
        },
        (Action::Move(Direction::Left), Place(_)) => match state.selection {
            Left(_) => Some(State { game: state.game, selection: Right((I1, I1)), error: None }),
            Right(square) => Some(State { game: state.game, selection: Right((square.0, prev(square.1).unwrap_or(square.1))), error: None }), 
        },
        (Action::Move(Direction::Right), Place(_)) => match state.selection {
            Left(_) => Some(State { game: state.game, selection: Right((I1, I1)), error: None }),
            Right(square) => Some(State { game: state.game, selection: Right((square.0, next(square.1).unwrap_or(square.1))), error: None }), 
        },
    }
}

fn play(game:Game, selection: Either<Piece, (Idx, Idx)>) -> Option<Game> {
    match (game, selection) {
        (Final(_), _) => None,
        (Pass(g), Left(p)) => g.pass(p).map(|x| x.to_game()),
        (Pass(_), _) => None,
        (Place(g), Right(square)) => g.place(square).map(|y|
            merge(bimap(y, |x| x.to_game(), |x| x.to_game()))
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

fn next(i: Idx) -> Option<Idx> {
    match i {
        I1 => Some(I2),
        I2 => Some(I3),
        I3 => Some(I4),
        I4 => None,
    }
}

fn prev(i: Idx) -> Option<Idx> {
    match i {
        I1 => None,
        I2 => Some(I1),
        I3 => Some(I2),
        I4 => Some(I3),
    }
}