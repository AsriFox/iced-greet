mod ui;
mod greeter;
mod query;
use iced::{ Application, Settings };

mod test;

fn main() {
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
}