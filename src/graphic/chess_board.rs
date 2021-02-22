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
        canvas.connect_draw(move |_, ctx| {
            ChessBoard::draw(ctx, cells_size);
            Inhibit(false)
        });

        Self { canvas }
    }

    pub fn get_canvas_ref(&self) -> &DrawingArea {
        &self.canvas
    }

    fn draw(context: &Context, cells_size: i32) {
        ChessBoard::draw_background(context);
        ChessBoard::draw_cells(context, cells_size);
    }

    fn draw_background(context: &Context) {
        let (bg_red, bg_green, bg_blue) = (214f64 / 255f64, 59f64 / 255f64, 96f64 / 255f64);
        context.set_source_rgb(bg_red, bg_green, bg_blue);
        context.paint();
    }

    fn draw_cells(context: &Context, cells_size: i32) {
        for row in 0..8 {
            for col in 0..8 {
                ChessBoard::set_cell_color(context, row, col);
                ChessBoard::paint_cell(context, row, col, cells_size);
            }
        }
    }

    fn set_cell_color(context: &Context, cell_row: i32, cell_col: i32) {
        let (white_cell_red, white_cell_green, white_cell_blue) =
            (255f64 / 255f64, 206f64 / 255f64, 158f64 / 255f64);
        let (black_cell_red, black_cell_green, black_cell_blue) =
            (209f64 / 255f64, 139f64 / 255f64, 71f64 / 255f64);

        let is_white_cell = (cell_row + cell_col) % 2 == 0;
        if is_white_cell {
            context.set_source_rgb(white_cell_red, white_cell_green, white_cell_blue);
        } else {
            context.set_source_rgb(black_cell_red, black_cell_green, black_cell_blue);
        }
    }

    fn paint_cell(context: &Context, cell_row: i32, cell_col: i32, cells_size: i32) {
        let cell_x = (cells_size as f64) * (0.5 + (cell_col as f64));
        let cell_y = (cells_size as f64) * (0.5 + (cell_row as f64));
        context.rectangle(cell_x, cell_y, cells_size as f64, cells_size as f64);
        context.fill();
    }
}
