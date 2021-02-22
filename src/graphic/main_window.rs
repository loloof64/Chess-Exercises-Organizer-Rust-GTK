use gtk::{ApplicationWindow};

use gtk::prelude::*;


use super::chess_board::ChessBoard;

pub struct MainWindow {
    window: ApplicationWindow,
    chess_board: ChessBoard,
}

impl MainWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = ApplicationWindow::new(app);
        window.set_title("Chess exercises organizer");
        window.set_default_size(350, 70);

        let chess_board = ChessBoard::new(60i32);

        MainWindow {
            window,
            chess_board,
        }
    }

    pub fn build(&self) {
        self.window.add(self.chess_board.get_canvas_ref());
    }

    pub fn show(&self) {
        self.window.show_all();
    }
}