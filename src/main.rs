extern crate gtk;
extern crate gio;

use gio::prelude::*;

use gtk::Application;

mod graphic;
use graphic::MainWindow;

fn main() {
    let application = Application::new(
        Some("com.loloof64.desktop.chess_exercises_organizer"),
        Default::default(),
    ).expect("Failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = MainWindow::new(app);
        window.build();
        window.show();
    });

    application.run(&[]);
}
