mod ui;
mod greeter;
use iced::{ Application, Settings };

fn main() {
    ui::GreetWindow::run(
        Settings {
            antialiasing: true,
            ..Settings::default()
        }
    ).unwrap();
}
