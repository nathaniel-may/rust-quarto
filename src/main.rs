mod common;
use crate::common::*;
mod splash;
mod local;

use structopt::StructOpt;
use std::{
    io::{Write, stdout},
    time::Duration,
};
use termion::{
    input::TermRead,
    raw::IntoRawMode,
};


static TICK_MS: Duration = Duration::from_millis(50); 

#[derive(StructOpt)]
struct Cli {
    // takes no arguments
}

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

        let _run_app = 
            // run the splash screen
            splash::SplashApp::run(&mut stdout, &mut stdin, TICK_MS)
                .and_then(|_| // TODO switch on mode here.
                    local::LocalApp::run(&mut stdout, &mut stdin, TICK_MS)
                ).and_then(|final_game|
                    // game is over. wait for user to quit.
                    local::DisplayWinnerApp::run_from(final_game, &mut stdout, &mut stdin, TICK_MS)
                );

        // cleanup terminal
        writeln!(stdout, "{}{}{}", 
            termion::cursor::Show,
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        ).unwrap();
    }
    println!("{}", "Done.")
}
