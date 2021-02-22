use cairo::Context;
use gtk::DrawingArea;

use gtk::prelude::*;

pub struct ChessBoard {
    canvas: DrawingArea,
}

impl ChessBoard {
    pub fn new(cells_size: i32) -> Self {
        let canvas = DrawingArea::new();
        let total_size = cells_size * 9;

        canvas.set_size_request(total_size, total_size);
        canvas.connect_draw(|_, ctx| {
            ChessBoard::draw(ctx);
            Inhibit(false)
        });

        Self { canvas }
    }

    pub fn get_canvas_ref(&self) -> &DrawingArea {
        &self.canvas
    }

    fn draw(context: &Context) {
        ChessBoard::draw_background(context);
    }

    fn draw_background(context: &Context) {
        let (bg_red, bg_green, bg_blue) = (214f64 / 255f64, 59f64 / 255f64, 96f64 / 255f64);
        context.set_source_rgb(bg_red, bg_green, bg_blue);
        context.paint();
    }
}
