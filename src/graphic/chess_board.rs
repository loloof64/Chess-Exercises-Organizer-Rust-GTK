use gtk::DrawingArea;

use gtk::prelude::*;

use super::chess_board_painter::ChessBoardPainter;

pub struct ChessBoard {
    canvas: DrawingArea,
}

impl ChessBoard {
    pub fn new(cells_size: i32) -> Self {
        let canvas = DrawingArea::new();
        let total_size = cells_size * 9;

        canvas.set_size_request(total_size, total_size);
        canvas.connect_draw(move |_, ctx| {
            ChessBoardPainter::draw(ctx, cells_size);
            Inhibit(false)
        });

        Self { canvas }
    }

    pub fn get_canvas_ref(&self) -> &DrawingArea {
        &self.canvas
    }

    
}
