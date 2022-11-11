mod ui;
mod greeter;
use iced::{ Application, Settings };

fn main() {
    ui::window::GreetWindow::run(
        Settings {
            antialiasing: true,
            ..Settings::default()
        }
    ).unwrap();
}
