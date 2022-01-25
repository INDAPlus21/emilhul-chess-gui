use super::*;
use ggez::graphics::Color;

/// ## draw_square
/// Helper function that draws a square to the screen by calling draw_rectangle.
/// The square size of the square is 1x1 GRID_CELL_SIZE.
/// The position of the square is given by x * GRID_CELL_SIZE and y * GRID_CELL_SIZE.
/// The color of the square is given by color
fn draw_square(ctx: &mut Context, x: f32, y: f32, color: Color) {
    draw_rectangle(ctx, x, y, 1.0, 1.0, color);
}

/// ## draw rectangle
/// Helper function that draws a rectangle to the screen.
/// The size of the rectangle is wxh GRID_CELL_SIZE.
/// The position of the rectangle is given by x * GRID_CELL_SIZE and y * GRID_CELL_SIZE.
/// The color of the rectangle is given by color.
fn draw_rectangle(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, color: Color) {
    let rectangle = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new_i32(
            (x * GRID_CELL_SIZE.0 as f32) as i32,
            (y * GRID_CELL_SIZE.1 as f32) as i32,
            (w * GRID_CELL_SIZE.0 as f32) as i32,
            (h * GRID_CELL_SIZE.1 as f32) as i32,
        ),
        color
    )
    .expect("Failed to create square.");

    graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
    .expect("Failed to draw highlight tile");
}

/// ## draw_sprite
/// Helper function for drawing a sprite to the screen.
/// The position is given as x * GRID_CELL_SIZE and y * GRID_CELL_SIZE.
/// Which sprite is drawn is decided by piece.
fn draw_sprite(appstate: &AppState, ctx: &mut Context, x: f32, y: f32, piece: Piece) {
    graphics::draw(
        ctx,
        match appstate.sprites.iter().find(|x| x.0 == piece) {
            Some(x) => &x.1,
            _ => panic!("No piece")
        },
        graphics::DrawParam::default()
            .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
            .dest([
                x * GRID_CELL_SIZE.0 as f32,
                y * GRID_CELL_SIZE.1 as f32,
            ]),
    )
    .expect("Failed to draw piece.");
}

/// ## draw_text
/// Helper function for drawing text to the screen.
/// The contents of the text is given by string. Which is then used to calculate the text_dimensions.
/// The position of the text is given by x * GRID_CELL_SIZE - text_dimension.w / 2.0 (to center the text)
/// and y * GRID_CELL_SIZE - text_dimension.h / 2.0 (to center the text).
/// The color of the text is given by color.
fn draw_text(ctx: &mut Context, x: f32, y: f32, color: Color, string: String) {
    let text = graphics::Text::new(
        graphics::TextFragment::from(string)
            .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
    );
    let text_dimensions = text.dimensions(ctx);
    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default()
            .color(color)
            .dest(ggez::mint::Point2 {
                x: x * GRID_CELL_SIZE.0 as f32 - text_dimensions.w / 2.0,
                y: y * GRID_CELL_SIZE.1 as f32 - text_dimensions.h / 2.0,
            }),
    )
    .expect("Failed to draw text.");
}

/// ## board
/// Draws the board and the pieces on it. Also draws highlights in case of highlighted moves or selected squares.
pub fn board(appstate: &AppState, ctx: &mut Context) {
    for _row in 0..8 {
        for _col in 0..8 {

            // Decide tile color
            let color = match _col % 2 {
                0 => {
                    if _row % 2 == 0 {
                        WHITE
                    } else {
                        BLACK
                    }
                },  
                _ => {
                    if _row % 2 == 0 {
                        BLACK
                    } else {
                        WHITE
                    }
                }
            };

            // Draw tile
            draw_square(ctx, _col as f32, _row as f32, color);

            // Draw highlighted_squares
            if appstate.highlighted_squares
            .contains(&(_row as usize, _col as usize)) { 
                draw_square(ctx, _col as f32, _row as f32, HIGHLIGHTED_COLOR); 
            }

            // Draw selected square
            if let Some(t) = appstate.selected_square {
                if _col == t.1 as i32 && _row == t.0 as i32 {
                    draw_square(ctx, _col as f32, _row as f32, SELECTED_COLOR);
                }
            }

            // Draw piece
            match appstate.game.board[_row as usize][_col as usize] {
                Piece::Empty => (),
                _piece => draw_sprite(appstate, ctx, _col as f32, _row as f32, _piece),
            }
        }
    }
}

/// ## promotion_selector
/// Draws the promotion selector on the right side of the screen. 
/// Highlights whichever piece is chosen for selection
pub fn promotion_selector(appstate: &AppState, ctx: &mut Context) {
    // Draw squares
    draw_square(ctx, 8.5, 1.0, BLACK);
    draw_square(ctx, 9.5, 1.0, WHITE);
    draw_square(ctx, 9.5, 2.0, BLACK);
    draw_square(ctx, 8.5, 2.0, WHITE);

    // Draw sprites
    draw_sprite(appstate, ctx, 8.5, 1.0, Piece::Queen(Colour::White));
    draw_sprite(appstate, ctx, 9.5, 1.0, Piece::Rook(Colour::White));
    draw_sprite(appstate, ctx, 8.5, 2.0, Piece::Bishop(Colour::White));
    draw_sprite(appstate, ctx, 9.5, 2.0, Piece::Knight(Colour::White));

    // Draw highlighted piece
    match appstate.game.selected_promotion {
        Piece::Queen(_colour) => draw_sprite(appstate, ctx, 8.5, 1.0, Piece::Queen(Colour::Black)),
        Piece::Rook(_colour) => draw_sprite(appstate, ctx, 9.5, 1.0, Piece::Rook(Colour::Black)),
        Piece::Bishop(_colour) => draw_sprite(appstate, ctx, 8.5, 2.0, Piece::Bishop(Colour::Black)),
        Piece::Knight(_colour) => draw_sprite(appstate, ctx, 9.5, 2.0, Piece::Knight(Colour::Black)),
        _ => panic!("No piece selected")
    }
}

/// ## history
/// Draws the history viewer on the right side of the screen.
pub fn history(appstate: &AppState, ctx: &mut Context) {
    // Draw history label text
    draw_text(ctx, 9.5, 3.25, WHITE, format!("History"));

    // Draw squares and count
    for i in 0..12 {
        // Draws squares
        draw_rectangle(
            ctx, 
            8.5 + 1.0 / 3.0, 
            3.5 + (i as f32) / 3.0, 
            2.0 / 3.0,
            1.0 / 3.0,
            match i % 2 {
                0 => WHITE,
                _ => BLACK,
            }
        );
        draw_rectangle(
            ctx, 
            9.5, 
            3.5 + (i as f32) / 3.0, 
            2.0 / 3.0,
            1.0 / 3.0,
            match i % 2 {
                0 => BLACK,
                _ => WHITE,
            }
        );

        // Draws turn numbers on left of history viewer
        draw_text(
            ctx, 
            8.5, 
            3.5 + (0.5 + i  as f32) / 3.0, 
            WHITE, 
            format!("{}", i + 1)
        );
    }

    // Draw out history markers in history viewer
    for i in 0..24 {
        if i < appstate.history.len() {
            match i % 2 {
                0 => draw_text(ctx, 9.0 + 1.0 / 6.0, 3.5 + (0.5 + i as f32 / 2.0) / 3.0, CONTRAST_COLOR, format!("{}", i + 1)),
                _ => draw_text(ctx, 9.0 + 5.0 / 6.0, 3.5 + (0.5 + (i as f32 / 2.0).floor()) / 3.0, CONTRAST_COLOR, format!("{}", i + 1)),
            }
        } else {
            break;
        }
    }
}
