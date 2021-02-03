mod common;
use crate::common::*;
mod splash;
mod local;
mod network;
use crate::splash::Mode;

use async_std::{
    io::IoSlice,
    prelude::*,
    net::TcpStream
};
use futures::executor::block_on;
use std::{
    io::{Write, stdout},
    time::Duration,
};
use serde_json::json;
use structopt::StructOpt;
use termion::{
    input::TermRead,
    raw::IntoRawMode,
};
use tide::{Request, Response};

use tide::prelude::*;

static TICK_MS: Duration = Duration::from_millis(50); 

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[derive(StructOpt)]
struct Cli {
    // takes no arguments
}

#[async_std::main]
async fn main() -> tide::Result<()> {

    // guards against passing arguments that won't be used.
    let _args = Cli::from_args();
    
    {
        let mut stdin = termion::async_stdin().keys();
        let mut stdout = stdout().into_raw_mode().unwrap();
    
        // prep the terminal
        write!(stdout, "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        ).unwrap();

        let _run_app = 
            // run the splash screen
            splash::SplashApp::run(&mut stdout, &mut stdin, TICK_MS)
                .and_then(|mode| 
                    match mode {
                        Mode::PassAndPlay => 
                            local::LocalApp::run(&mut stdout, &mut stdin, TICK_MS),
                        Mode::LocalNetwork => 
                            network::NetworkApp::run(&mut stdout, &mut stdin, TICK_MS)
                    }
                ).and_then(|final_game|
                    // game is over. wait for user to quit.
                    // TODO might have to write a networked version so it can send and display on both clients.
                    local::DisplayWinnerApp::run_from(final_game, &mut stdout, &mut stdin, TICK_MS)
                );

        // cleanup terminal
        writeln!(stdout, "{}{}{}", 
            termion::cursor::Show,
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        ).unwrap();
    }
    println!("{}", "Done.");
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    let peer_addr = req.peer_addr().ok_or("no peer address provided");
    // let mut stream = TcpStream::connect(peer_addr).await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
