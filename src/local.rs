use crate::common::*;
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
pub enum Player {
    P1,
    P2,
}

impl Player {
    fn switch(&self) -> Player {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub struct State {
    game: Game,
    player: Player,
    selection: Either<(bool, usize), (Idx, Idx)>,
    error: Option<&'static str>,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Action {
    Quit,
    Submit,
    Move(Direction), 
    Idle,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}


pub struct LocalApp {}

impl App for LocalApp {
    type State = State;
    type Action = Action;
    type Output = State;

    fn initial_state() -> Self::State {
        State {
            game: quarto::new_game().to_game(),
            player: Player::P1,
            selection: Left((true, 0)),
            error: None,
        }
    }

    fn write_state<W: io::Write>(f: &mut W, state: Self::State) {
        // clear all output
        f.write_fmt(format_args!("{}", termion::clear::All)).unwrap();

        write_banner_at((1, 2), f);

        let mut cursor: (u16, u16) = (8, 8);

        // write descriptor string
        let player_str = match state.player {
            Player::P1 => String::from("P1"),
            Player::P2 => String::from("P2"),
        };
        let descriptor = match state.game {
            g@Final(_) if g.is_tie() => String::from("    Tie Game!!!     "), //11
            Final(_) => String::from("     ") + &(player_str + " Wins!!!     "), //10
            Pass(_)  => String::from(" ") + &(player_str + ", Pass a piece.  "), //17
            Place(_)  => player_str + &(", Place the piece."), //20
            //36
        };
        write_at(cursor, f, &descriptor);
        cursor.1 += 2;

        // write game board out
        cursor.0 = 6;
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
            let selected = (either::Left(piece_cursor) == state.selection) && !state.game.is_final();
            write_piece(f, &available_piece, selected);
            f.write_fmt(format_args!(" ")).unwrap();
            piece_cursor.1 += 1;
        }
        cursor.1 += 1;

        // write pass menu row 2
        piece_cursor = (false, 0);
        f.write_fmt(format_args!("{}", termion::cursor::Goto(cursor.0, cursor.1))).unwrap();
        for p in &ALL_PIECES[8..] {
            let available_piece = if state.game.contains(p) { None } else { Some(*p) };
            let selected = (either::Left(piece_cursor) == state.selection) && !state.game.is_final();
            write_piece(f, &available_piece, selected);
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

        // write "any key to exit" on a final game
        cursor.0 = 8;
        match state.game {
            Final(_) => write_at(cursor, f, "press any key to exit."),
            _        => {},
        }
    }

    fn action_from(key: Option<std::result::Result<termion::event::Key, std::io::Error>>) -> Self::Action {
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
    
    fn step(state: Self::State, action: Self::Action) -> Option<Self::State> {
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
                    None => Some(State { game: state.game, player: state.player, selection: state.selection, error: Some("try again.") }),
                    Some(g@Place(_)) => Some(State { game: g, player: state.player.switch(), selection: new_cursor, error: state.error }),
                    Some(g) => Some(State { game: g, player: state.player, selection: new_cursor, error: state.error }),
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
                Right(square) => Some(State { game: state.game, player: state.player, selection: Right((prev(square.0).unwrap_or(square.0), square.1)), error: None }), 
            },
            (Action::Move(Direction::Down), Place(_)) => match state.selection {
                Left(_) => Some(State { game: state.game, player: state.player, selection: Right((I1, I1)), error: None }),
                Right(square) => Some(State { game: state.game, player: state.player, selection: Right((next(square.0).unwrap_or(square.0), square.1)), error: None }), 
            },
            (Action::Move(Direction::Left), Place(_)) => match state.selection {
                Left(_) => Some(State { game: state.game, player: state.player, selection: Right((I1, I1)), error: None }),
                Right(square) => Some(State { game: state.game, player: state.player, selection: Right((square.0, prev(square.1).unwrap_or(square.1))), error: None }), 
            },
            (Action::Move(Direction::Right), Place(_)) => match state.selection {
                Left(_) => Some(State { game: state.game, player: state.player, selection: Right((I1, I1)), error: None }),
                Right(square) => Some(State { game: state.game, player: state.player, selection: Right((square.0, next(square.1).unwrap_or(square.1))), error: None }), 
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

pub enum Void {}

pub struct DisplayWinnerApp {}

impl App for DisplayWinnerApp {
    type State = State;
    type Action = bool;
    type Output = Void;

    fn initial_state() -> Self::State {
        State {
            game: quarto::new_game().to_game(),
            player: Player::P1,
            selection: Left((true, 0)),
            error: None,
        }
    }

    fn write_state<W: io::Write>(f: &mut W, state: Self::State) {
        LocalApp::write_state(f, state)
    }

    fn action_from(key: Option<std::result::Result<termion::event::Key, std::io::Error>>) -> Self::Action {
        match key {
            Some(Ok(_)) => true,
            _ => false,
        }
    }
    
    fn step(state: Self::State, action: Self::Action) -> Option<Self::State> {
        match (action, state) {
            (true, _) => None,
            (false, s) => Some(s),
        }
    }

    fn output_from(_state: Self::State) -> Option<Self::Output> {
        None // no output. waits for user to quit.
    }
}

fn write_piece<W: io::Write>(f: &mut W, op: &Option<Piece>, selected: bool) {
    match op {
        None => {
            if selected {
                f.write_fmt(format_args!("{bg}{empty}{reset_bg}",
                    bg = color::Bg(color::AnsiValue::grayscale(12)),
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
                Color::White => color::Fg(color::AnsiValue::rgb(5, 0, 0)),
                Color::Black => color::Fg(color::AnsiValue::rgb(0, 1, 5)),
            };

            // directly match poorly designed termion types
            match (p.height, selected) {
                (Height::Tall, true)  => {
                    f.write_fmt(format_args!("{bold}{c}{bg}{u}{piece}{reset_color}{reset_bg}{reset_style}",
                        bold = style::Bold,
                        c = color,
                        bg = color::Bg(color::AnsiValue::grayscale(12)),
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
                        bg = color::Bg(color::AnsiValue::grayscale(12)),
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