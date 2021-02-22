use gtk::{ApplicationWindow, Button};

use gtk::prelude::*;

pub struct MainWindow {
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = ApplicationWindow::new(app);
        window.set_title("Chess exercises organizer");
        window.set_default_size(350, 70);

        MainWindow {
            window,
        }
    }

    pub fn build(&self) {
        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        self.window.add(&button);
    }

    pub fn show(&self) {
        self.window.show_all();
    }
}