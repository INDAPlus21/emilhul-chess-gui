use super::*;
use ggez::graphics::Color;

fn draw_square(ctx: &mut Context, x: f32, y: f32, color: Color) {
    let rectangle = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new_i32(
            (x * GRID_CELL_SIZE.0 as f32) as i32,
            (y * GRID_CELL_SIZE.1 as f32) as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        ),
        color
    )
    .expect("Failed to create square.");

    graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
    .expect("Failed to draw highlight tile");
}

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

    // Draw highlighted square
    match appstate.game.selected_promotion {
        Piece::Queen(_colour) => draw_sprite(appstate, ctx, 8.5, 1.0, Piece::Queen(Colour::Black)),
        Piece::Rook(_colour) => draw_sprite(appstate, ctx, 9.5, 1.0, Piece::Rook(Colour::Black)),
        Piece::Bishop(_colour) => draw_sprite(appstate, ctx, 8.5, 2.0, Piece::Bishop(Colour::Black)),
        Piece::Knight(_colour) => draw_sprite(appstate, ctx, 9.5, 2.0, Piece::Knight(Colour::Black)),
        _ => panic!("No piece selected")
    }
}

pub fn history(appstate: &AppState, ctx: &mut Context) {
    // Draw the history
    // Draw history label text
    let history_text = graphics::Text::new(
        graphics::TextFragment::from(format!("History"))
            .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
    );
    let history_text_dimensions = history_text.dimensions(ctx);
    graphics::draw(
        ctx,
        &history_text,
        graphics::DrawParam::default()
            .color(WHITE)
            .dest(ggez::mint::Point2 {
                x: (GRID_CELL_SIZE.0 as f32 * GRID_SIZE as f32)
                    + (270f32 - history_text_dimensions.w as f32) / 2f32,
                y: (GRID_CELL_SIZE.1 as f32 * 3f32) + (GRID_CELL_SIZE.1 as f32 / 8f32),
            }),
    )
    .expect("Failed to draw text.");

    // Draw squares and count
    for i in 0..12 {
        // Draws squares
        draw_rectangle(ctx, 
            8.5 + 1.0 / 3.0, 
            3.5 + (i as f32) / 3.0, 
            2.0 / 3.0,
            1.0 / 3.0,
            match i % 2 {
                0 => WHITE,
                _ => BLACK,
            }
        );
        draw_rectangle(ctx, 
            9.5, 
            3.5 + (i as f32) / 3.0, 
            2.0 / 3.0,
            1.0 / 3.0,
            match i % 2 {
                0 => BLACK,
                _ => WHITE,
            }
        );

        /* Draw count
        let history_text = graphics::Text::new(
            graphics::TextFragment::from(format!("{:?}", i + 1))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );
        let history_text_dimensions = history_text.dimensions(ctx);
        graphics::draw(
            ctx,
            &history_text,
            graphics::DrawParam::default()
                .color(WHITE)
                .dest(ggez::mint::Point2 {
                    x: (GRID_CELL_SIZE.0 as f32 * GRID_SIZE as f32)
                        + (GRID_CELL_SIZE.0 as f32 * 1.5f32
                            - history_text_dimensions.w as f32)
                            / 3f32,
                    y: (GRID_CELL_SIZE.1 as f32 * 3.5f32)
                        + (GRID_CELL_SIZE.1 as f32 * i as f32) / 3f32,
                }),
        )
        .expect("Failed to draw text.");*/
    }
    // Draw out history text
    for i in 0..24 {
        if i < appstate.history.len() {
            match i % 2 {
                0 => {
                    let history_text = graphics::Text::new(
                        graphics::TextFragment::from(format!("{:?}", i + 1))
                            .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
                    );
                    let history_text_dimensions = history_text.dimensions(ctx);
                    graphics::draw(
                        ctx,
                        &history_text,
                        graphics::DrawParam::default().color(CONTRAST_COLOR).dest(
                            ggez::mint::Point2 {
                                x: (GRID_CELL_SIZE.0 as f32 * GRID_SIZE as f32)
                                    + (105f32 - history_text_dimensions.w / 2f32),
                                y: (GRID_CELL_SIZE.1 as f32 * 3.5f32)
                                    + (GRID_CELL_SIZE.1 as f32 * ((i / 2) as f32).ceil())
                                        / 3f32,
                            },
                        ),
                    )
                    .expect("Failed to draw text.");
                }
                1 => {
                    let history_text = graphics::Text::new(
                        graphics::TextFragment::from(format!("{:?}", i + 1))
                            .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
                    );
                    let history_text_dimensions = history_text.dimensions(ctx);
                    graphics::draw(
                        ctx,
                        &history_text,
                        graphics::DrawParam::default().color(CONTRAST_COLOR).dest(
                            ggez::mint::Point2 {
                                x: (GRID_CELL_SIZE.0 as f32 * GRID_SIZE as f32)
                                    + (165f32 - history_text_dimensions.w / 2f32),
                                y: (GRID_CELL_SIZE.1 as f32 * 3.5f32)
                                    + (GRID_CELL_SIZE.1 as f32 * ((i / 2) as f32).ceil())
                                        / 3f32,
                            },
                        ),
                    )
                    .expect("Failed to draw text.");
                }
                _ => panic!("How 2?"),
            }
        } else {
            break;
        }
    }
}
