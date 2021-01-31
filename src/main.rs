mod common;
use crate::common::*;
mod splash;
mod local;

use async_std::{
    io::IoSlice,
    prelude::*,
    net::TcpStream
};
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



static TICK_MS: Duration = Duration::from_millis(50); 

#[derive(StructOpt)]
struct Cli {
    // takes no arguments
}

#[async_std::main]
async fn main() -> tide::Result<()> {

    // guards against passing arguments that won't be used.
    let _args = Cli::from_args();

    let mut app = tide::new();
    app.at("/adoption").post(create_cert);
    app.listen(vec!["[::]:8080", "0.0.0.0:5001"]).await?;
    
    // {
    //     let mut stdin = termion::async_stdin().keys();
    //     let mut stdout = stdout().into_raw_mode().unwrap();
    
    //     // prep the terminal
    // write!(stdout, "{}{}{}",
    //     termion::clear::All,
    //     termion::cursor::Goto(1, 1),
    //     termion::cursor::Hide).unwrap();

    // let _run_app = 
    //     // run the splash screen
    //     splash::SplashApp::run(&mut stdout, &mut stdin, TICK_MS)
    //         .and_then(|_| // TODO switch on mode here.
    //             local::LocalApp::run(&mut stdout, &mut stdin, TICK_MS)
    //         ).and_then(|final_game|
    //             // game is over. wait for user to quit.
    //             local::DisplayWinnerApp::run_from(final_game, &mut stdout, &mut stdin, TICK_MS)
    //         );

    //     // cleanup terminal
    //     writeln!(stdout, "{}{}{}", 
    //         termion::cursor::Show,
    //         termion::clear::All,
    //         termion::cursor::Goto(1, 1)
    //     ).unwrap();
    // }
    // println!("{}", "Done.");
    Ok(())
}

async fn create_cert(mut req: Request<()>) -> tide::Result {
    let mut vec = req.body_bytes().await?;
    let mut stream = TcpStream::connect("127.0.0.1:7979").await?;
    let len = [vec.len() as u8];
    stream.write_vectored(&mut [IoSlice::new(&len), IoSlice::new(&vec)]).await?;
    vec.clear();
    stream.read_to_end(&mut vec).await?;
    let encoded = base64::encode(&vec[1..]);
    Ok(Response::from(json!({"certificate": encoded})))
}
