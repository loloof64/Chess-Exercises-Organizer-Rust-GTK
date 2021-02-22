use cairo::{Context, FontFace, FontWeight};
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
        ChessBoard::draw_coordinates(context, cells_size);
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

    fn draw_coordinates(context: &Context, cells_size: i32) {
        let ascii_uppercase_a = 65u8;
        let ascii_1 = 49u8;
        let coordinates_color = (255f64 / 255f64, 199f64 / 255f64, 0f64);

        let font_size = 0.35 * cells_size as f64;
        let old_font_face = context.get_font_face();

        context.set_source_rgb(
            coordinates_color.0,
            coordinates_color.1,
            coordinates_color.2,
        );
        context.set_font_size(font_size);
        context.set_font_face(&FontFace::toy_create(
            old_font_face
                .toy_get_family()
                .expect("Failed to get current font family")
                .as_str(),
            old_font_face.toy_get_slant(),
            FontWeight::Bold,
        ));

        for col in 0..8 {
            let file = col;
            let coordinate = (ascii_uppercase_a + file) as char;
            let coordinate = format!("{}", coordinate);
            let coordinate = coordinate.as_str();

            let x = (cells_size as f64) * (0.9 + col as f64);
            let y1 = (cells_size as f64) * 0.35;
            let y2 = (cells_size as f64) * 8.85;

            context.move_to(x, y1);
            context.show_text(coordinate);

            context.move_to(x, y2);
            context.show_text(coordinate);
        }

        for row in 0..8 {
            let rank = 7 - row;
            let coordinate = (ascii_1 + rank) as char;
            let coordinate = format!("{}", coordinate);
            let coordinate = coordinate.as_str();

            let y = (cells_size as f64) * (1.2 + row as f64);
            let x1 = (cells_size as f64) * 0.15;
            let x2 = (cells_size as f64) * 8.65;

            context.move_to(x1, y);
            context.show_text(coordinate);

            context.move_to(x2, y);
            context.show_text(coordinate);
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
