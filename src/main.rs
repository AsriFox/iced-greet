//! ## iced-greet
//! 
//! Greeter window for greetd built with iced
//! 
//! Command-line arguments:
//! - *test* - run the testing version (without connecting to greetd socket) 

mod ui;
mod greeter;
mod query;
use iced::{ Application, Settings };

use std::fs::OpenOptions;

mod test;

fn main() {
    let log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/iced-greet.log")
        .unwrap();
    let gagerr = gag::Redirect::stderr(log).unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "test" {
        <test::TestWindow as iced::Sandbox>::run(
            Settings {
                antialiasing: true,
                ..Settings::default()
            }
        ).unwrap();
    }
    else {
        ui::GreetWindow::run(
            Settings {
                antialiasing: true,
                ..Settings::default()
            }
        ).unwrap();
    }

    drop(gagerr)
}