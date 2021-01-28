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

        // print banner
        for (idx, line) in BANNER.iter().enumerate() {
            let i = (idx + 1) as u16;
            write!(
                stdout, "{}{}",
                termion::cursor::Goto(1, i),
                line
            ).unwrap();
        }

        write!(stdout, "{}q to exit. <Enter> to continue.",
            termion::cursor::Goto(1, 8)).unwrap();

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
                    Key::Char('\n') => break,
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
fn write<W: io::Write>(f: &mut W, s: &str) {
    f.write_fmt(format_args!("{}", s)).unwrap();
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

fn render<A: Display>(content: A) {
    fn render0<A: Display>(content: A) -> Result<(), io::Error> {
        let mut screen = AlternateScreen::from(stdout());
        writeln!(screen, "{}", content)?;
        screen.flush()
    }

    render0(content).unwrap()
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
    render("game"); // TODO stub
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
                write(output, "tie game!!!")
            } else {
                write(output, "you win!!!")
            }
            std::process::exit(1)
        },
        (Action::Move(Direction::Up), Pass(_)) => run(output, input, state),
        (Action::Move(Direction::Down), Pass(_)) => run(output, input, state),
        (Action::Move(Direction::Left), Pass(_)) => match state.selection {
            Left(i) => run(output, input, State { game: state.game, selection: Left(min(0, i-1)), error: None }),
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