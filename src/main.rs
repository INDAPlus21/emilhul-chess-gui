use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult};
use murnion_chess::{Colour, Game, Piece};
use std::path;

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32 + 270f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color =
    graphics::Color::new(188f32 / 255f32, 140f32 / 255f32, 76f32 / 255f32, 1f32);
const WHITE: graphics::Color =
    graphics::Color::new(255f32 / 255f32, 200f32 / 255f32, 128f32 / 255f32, 1f32);
const SELECTED_COLOR: graphics::Color =
    graphics::Color::new(22f32 / 255f32, 80f32 / 255f32, 112f32 / 255f32, 1f32);
const HIGHLIGHTED_COLOR: graphics::Color =
    graphics::Color::new(75f32 / 255f32, 148f32 / 255f32, 189f32 / 255f32, 0.8f32);
const CONTRAST_COLOR: graphics::Color =
    graphics::Color::new(112f32 / 255f32, 78f32 / 255f32, 34f32 / 255f32, 1f32);
const CERISE: graphics::Color =
    graphics::Color::new(222f32 / 255f32, 49f32 / 255f32, 99f32 / 255f32, 0.15f32);

/// GUI logic and event implementation structure.
struct AppState {
    sprites: Vec<(Piece, graphics::Image)>,
    game: Game, // Save piece positions, which tiles has been clicked, current colour, etc...
    selected_square: Option<(usize, usize)>,
    highlighted_squares: Vec<(usize, usize)>,
    history: Vec<String>, //A vector containing all previous game states as FEN strings
    viewing_history: bool,
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let state = AppState {
            sprites: AppState::load_sprites(ctx),
            game: Game::new(),
            selected_square: None,
            highlighted_squares: Vec::new(),
            history: Vec::new(),
            viewing_history: false,
        };

        Ok(state)
    }

    /// Loads chess piese images into vector.
    fn load_sprites(ctx: &mut Context) -> Vec<(Piece, graphics::Image)> {
        [
            (Piece::King(Colour::Black), "/black_king.png".to_string()),
            (Piece::Queen(Colour::Black), "/black_queen.png".to_string()),
            (Piece::Rook(Colour::Black), "/black_rook.png".to_string()),
            (Piece::Pawn(Colour::Black), "/black_pawn.png".to_string()),
            (
                Piece::Bishop(Colour::Black),
                "/black_bishop.png".to_string(),
            ),
            (
                Piece::Knight(Colour::Black),
                "/black_knight.png".to_string(),
            ),
            (Piece::King(Colour::White), "/white_king.png".to_string()),
            (Piece::Queen(Colour::White), "/white_queen.png".to_string()),
            (Piece::Rook(Colour::White), "/white_rook.png".to_string()),
            (Piece::Pawn(Colour::White), "/white_pawn.png".to_string()),
            (
                Piece::Bishop(Colour::White),
                "/white_bishop.png".to_string(),
            ),
            (
                Piece::Knight(Colour::White),
                "/white_knight.png".to_string(),
            ),
        ]
        .iter()
        .map(|(_piece, _path)| (*_piece, graphics::Image::new(ctx, _path).unwrap()))
        .collect::<Vec<(Piece, graphics::Image)>>()
    }
}

impl event::EventHandler<GameError> for AppState {
    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear interface with gray background colour
        graphics::clear(ctx, CONTRAST_COLOR);

        // draw grid
        for _row in 0..8 {
            for _col in 0..8 {
                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        _col * GRID_CELL_SIZE.0 as i32,
                        _row * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    match _col % 2 {
                        0 => {
                            if _row % 2 == 0 {
                                WHITE
                            } else {
                                BLACK
                            }
                        }
                        _ => {
                            if _row % 2 == 0 {
                                BLACK
                            } else {
                                WHITE
                            }
                        }
                    },
                )
                .expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw tiles.");

                // Draw highlighted_squares
                if self
                    .highlighted_squares
                    .contains(&(_row as usize, _col as usize))
                {
                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new_i32(
                            _col * GRID_CELL_SIZE.0 as i32,
                            _row * GRID_CELL_SIZE.1 as i32,
                            GRID_CELL_SIZE.0 as i32,
                            GRID_CELL_SIZE.1 as i32,
                        ),
                        HIGHLIGHTED_COLOR,
                    )
                    .expect("Failed to create highlight tile.");
                    graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                        .expect("Failed to draw highlight tile");
                }

                // Draw selected square
                if let Some(t) = self.selected_square {
                    if _col == t.1 as i32 && _row == t.0 as i32 {
                        let rectangle = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect::new_i32(
                                _col * GRID_CELL_SIZE.0 as i32,
                                _row * GRID_CELL_SIZE.1 as i32,
                                GRID_CELL_SIZE.0 as i32,
                                GRID_CELL_SIZE.1 as i32,
                            ),
                            SELECTED_COLOR,
                        )
                        .expect("Failed to create selected tile.");
                        graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                            .expect("Failed to draw selected tile");
                    }
                }
                // Draw piece
                match self.game.board[_row as usize][_col as usize] {
                    Piece::Empty => (),
                    _piece => {
                        let sprite = match self.sprites.iter().find(|x| x.0 == _piece) {
                            Some(x) => x.1.clone(),
                            _ => panic!("No piece"),
                        };
                        graphics::draw(
                            ctx,
                            &sprite,
                            graphics::DrawParam::default()
                                .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                                .dest([
                                    _col as f32 * GRID_CELL_SIZE.0 as f32,
                                    _row as f32 * GRID_CELL_SIZE.1 as f32,
                                ]),
                        )
                        .expect("Failed to draw piece.");
                    }
                }
            }
        }

        {
            //Draws promotion selector
            {
                //Draws squares
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        (8f32 * GRID_CELL_SIZE.0 as f32 + 0.5f32 * GRID_CELL_SIZE.0 as f32) as i32,
                        GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    BLACK,
                )
                .expect("Failed to create highlight tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw highlight tile");
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        (9f32 * GRID_CELL_SIZE.0 as f32 + 0.5f32 * GRID_CELL_SIZE.0 as f32) as i32,
                        GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    WHITE,
                )
                .expect("Failed to create highlight tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw highlight tile");
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        (9f32 * GRID_CELL_SIZE.0 as f32 + 0.5f32 * GRID_CELL_SIZE.0 as f32) as i32,
                        2 * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    BLACK,
                )
                .expect("Failed to create highlight tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw highlight tile");
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        (8f32 * GRID_CELL_SIZE.0 as f32 + 0.5f32 * GRID_CELL_SIZE.0 as f32) as i32,
                        2 * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    WHITE,
                )
                .expect("Failed to create highlight tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw highlight tile");
            }
            {
                // Draws pieces. Color of the pieces changes depending on which piece is selected for promotion.
                let sprite = match self.sprites.iter().find(|x| {
                    x.0 == Piece::Queen(match self.game.selected_promotion {
                        Piece::Queen(_colour) => Colour::Black,
                        _ => Colour::White,
                    })
                }) {
                    Some(x) => x.1.clone(),
                    _ => panic!("No piece"),
                };
                graphics::draw(
                    ctx,
                    &sprite,
                    graphics::DrawParam::default()
                        .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                        .dest([
                            8f32 * GRID_CELL_SIZE.0 as f32 + 0.5f32 * GRID_CELL_SIZE.0 as f32,
                            GRID_CELL_SIZE.1 as f32,
                        ]),
                )
                .expect("Failed to draw piece.");
                let sprite = match self.sprites.iter().find(|x| {
                    x.0 == Piece::Rook(match self.game.selected_promotion {
                        Piece::Rook(_colour) => Colour::Black,
                        _ => Colour::White,
                    })
                }) {
                    Some(x) => x.1.clone(),
                    _ => panic!("No piece"),
                };
                graphics::draw(
                    ctx,
                    &sprite,
                    graphics::DrawParam::default()
                        .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                        .dest([
                            9f32 * GRID_CELL_SIZE.0 as f32 + 0.5f32 * GRID_CELL_SIZE.0 as f32,
                            GRID_CELL_SIZE.1 as f32,
                        ]),
                )
                .expect("Failed to draw piece.");
                let sprite = match self.sprites.iter().find(|x| {
                    x.0 == Piece::Bishop(match self.game.selected_promotion {
                        Piece::Bishop(_colour) => Colour::Black,
                        _ => Colour::White,
                    })
                }) {
                    Some(x) => x.1.clone(),
                    _ => panic!("No piece"),
                };
                graphics::draw(
                    ctx,
                    &sprite,
                    graphics::DrawParam::default()
                        .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                        .dest([
                            8f32 * GRID_CELL_SIZE.0 as f32 + 0.5f32 * GRID_CELL_SIZE.0 as f32,
                            2f32 * GRID_CELL_SIZE.1 as f32,
                        ]),
                )
                .expect("Failed to draw piece.");
                let sprite = match self.sprites.iter().find(|x| {
                    x.0 == Piece::Knight(match self.game.selected_promotion {
                        Piece::Knight(_colour) => Colour::Black,
                        _ => Colour::White,
                    })
                }) {
                    Some(x) => x.1.clone(),
                    _ => panic!("No piece"),
                };
                graphics::draw(
                    ctx,
                    &sprite,
                    graphics::DrawParam::default()
                        .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                        .dest([
                            9f32 * GRID_CELL_SIZE.0 as f32 + 0.5f32 * GRID_CELL_SIZE.0 as f32,
                            2f32 * GRID_CELL_SIZE.1 as f32,
                        ]),
                )
                .expect("Failed to draw piece.");
            }
        }
        {
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
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        (8f32 * GRID_CELL_SIZE.0 as f32
                            + 0.5f32 * GRID_CELL_SIZE.0 as f32
                            + GRID_CELL_SIZE.0 as f32 / 3f32) as i32,
                        (GRID_CELL_SIZE.1 as f32 * 3.5f32
                            + (GRID_CELL_SIZE.1 as f32 * i as f32) / 3f32)
                            as i32,
                        (GRID_CELL_SIZE.0 as f32 * 2f32 / 3f32) as i32,
                        (GRID_CELL_SIZE.1 as f32 / 3f32) as i32,
                    ),
                    match i % 2 {
                        0 => WHITE,
                        1 => BLACK,
                        _ => panic!("How???"),
                    },
                )
                .expect("Failed to create history background.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw history background");
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        (8f32 * GRID_CELL_SIZE.0 as f32 + 1.5f32 * GRID_CELL_SIZE.0 as f32) as i32,
                        (GRID_CELL_SIZE.1 as f32 * 3.5f32
                            + (GRID_CELL_SIZE.1 as f32 * i as f32) / 3f32)
                            as i32,
                        (GRID_CELL_SIZE.0 as f32 * 2f32 / 3f32) as i32,
                        (GRID_CELL_SIZE.1 as f32 / 3f32) as i32,
                    ),
                    match i % 2 {
                        0 => BLACK,
                        1 => WHITE,
                        _ => panic!("How???"),
                    },
                )
                .expect("Failed to create history background.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw history background");

                // Draw count
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
                .expect("Failed to draw text.");
            }
            // Draw out history text
            for i in 0..24 {
                if i < self.history.len() {
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
        // create text representation
        let turn_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Turn: {:?}", self.game.turn))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );
        let color_text = graphics::Text::new(
            graphics::TextFragment::from(format!("{:?} to move", self.game.current_turn))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );
        let color_text_dimensions = color_text.dimensions(ctx);
        // get size of text
        let text_dimensions = turn_text.dimensions(ctx);

        // draw text with light colouring in the ofboard part
        graphics::draw(
            ctx,
            &turn_text,
            graphics::DrawParam::default()
                .color(WHITE)
                .dest(ggez::mint::Point2 {
                    x: (GRID_CELL_SIZE.0 as f32 * GRID_SIZE as f32)
                        + (270f32 - text_dimensions.w as f32) / 2f32,
                    y: ((GRID_CELL_SIZE.1 as f32 / 2f32) + text_dimensions.h as f32) / 2f32,
                }),
        )
        .expect("Failed to draw text.");

        graphics::draw(
            ctx,
            &color_text,
            graphics::DrawParam::default()
                .color(WHITE)
                .dest(ggez::mint::Point2 {
                    x: (GRID_CELL_SIZE.0 as f32 * GRID_SIZE as f32)
                        + (270f32 - color_text_dimensions.w as f32) / 2f32,
                    y: ((GRID_CELL_SIZE.1 as f32 / 2f32) - color_text_dimensions.h as f32) / 2f32,
                }),
        )
        .expect("Failed to draw text.");

        if self.viewing_history {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(0, 0, SCREEN_SIZE.0 as i32, SCREEN_SIZE.1 as i32),
                CERISE,
            )
            .expect("Failed to create history background.");
            graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                .expect("Failed to draw history background");
        }

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == event::MouseButton::Left {
            /* check click position and update board accordingly */
            if x < (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32) {
                let rank = (y / 90f32).floor() as usize;
                let file = (x / 90f32).floor() as usize;

                match self.selected_square {
                    Some(t) => {
                        if self.highlighted_squares.contains(&(rank, file)) {
                            if !self.viewing_history {
                                self.history.push(self.game.get_fen());
                                self.game
                                    .take_turn(move_to_string((t.0, t.1), (rank, file)));
                            }
                            self.selected_square = None;
                            self.highlighted_squares = Vec::new();
                        } else if (rank, file) == t {
                            self.selected_square = None;
                            self.highlighted_squares = Vec::new();
                        } else {
                            self.selected_square = Some((rank, file));
                            self.highlighted_squares = Vec::new();
                            if let Some(c) = get_colour(self.game.board[rank][file]) {
                                if c == self.game.current_turn {
                                    self.highlighted_squares = self.game.board[rank][file]
                                        .get_valid_moves(
                                            (rank, file),
                                            &self.game.board,
                                            self.game.en_passant_square,
                                            self.game.castlings,
                                            self.game.current_turn,
                                        );
                                };
                            };
                        }
                    }
                    None => {
                        // Tuple (rank, file)
                        self.selected_square = Some((rank, file));
                        self.highlighted_squares = Vec::new();
                        if let Some(c) = get_colour(self.game.board[rank][file]) {
                            if c == self.game.current_turn {
                                self.highlighted_squares = self.game.board[rank][file]
                                    .get_valid_moves(
                                        (rank, file),
                                        &self.game.board,
                                        self.game.en_passant_square,
                                        self.game.castlings,
                                        self.game.current_turn,
                                    );
                            };
                        };
                    }
                }
            } else if x > ((GRID_SIZE as f32 + 0.5f32) * GRID_CELL_SIZE.0 as f32)
                && x < ((GRID_SIZE as f32 + 2.5f32) * GRID_CELL_SIZE.0 as f32)
                && y > GRID_CELL_SIZE.1 as f32
                && y < (3f32 * GRID_CELL_SIZE.1 as f32)
            {
                let rank = (y / 90f32 - 1f32).floor() as usize;
                let file = (x / 90f32 - 8.5f32).floor() as usize;
                match (rank, file) {
                    (0, 0) => self.game.select_promotion('q'),
                    (0, 1) => self.game.select_promotion('r'),
                    (1, 0) => self.game.select_promotion('b'),
                    (1, 1) => self.game.select_promotion('n'),
                    _ => panic!("Not a promotion piece"),
                }
            } else if x > ((GRID_SIZE as f32 + 0.5f32 + 1f32 / 3f32) * GRID_CELL_SIZE.0 as f32)
                && x < ((GRID_SIZE as f32 + 0.5f32 + 5f32 / 3f32) * GRID_CELL_SIZE.0 as f32)
                && y > GRID_CELL_SIZE.1 as f32 * 3.5f32
                && y < (3.5f32 + 4f32) * GRID_CELL_SIZE.1 as f32
            {
                let rank = (y / 90f32 * 3f32 - 10.5f32).floor() as usize;
                let file = (x / 90f32 - 8.5f32).floor() as usize;
                let index: usize = match (rank, file) {
                    (0, 0) => 0,
                    (0, 1) => 1,
                    (1, 0) => 2,
                    (1, 1) => 3,
                    (2, 0) => 4,
                    (2, 1) => 5,
                    (3, 0) => 6,
                    (3, 1) => 7,
                    (4, 0) => 8,
                    (4, 1) => 9,
                    (5, 0) => 10,
                    (5, 1) => 11,
                    (6, 0) => 12,
                    (6, 1) => 13,
                    (7, 0) => 14,
                    (7, 1) => 15,
                    (8, 0) => 16,
                    (8, 1) => 17,
                    (9, 0) => 18,
                    (9, 1) => 19,
                    (10, 0) => 20,
                    (10, 1) => 21,
                    (11, 0) => 22,
                    (11, 1) => 23,
                    _ => panic!("Out of bounds!"),
                };
                if index <= self.history.len() && !self.viewing_history {
                    self.history.push(self.game.get_fen());
                    self.game.set_state_from_fen(&self.history[index]);
                    self.viewing_history = true;
                } else if index < self.history.len() - 1 && self.viewing_history {
                    self.game.set_state_from_fen(&self.history[index]);
                } else if index == self.history.len() - 1 && self.viewing_history {
                    if let Some(current_turn) = self.history.pop() {
                        self.game.set_state_from_fen(&current_turn);
                        self.viewing_history = false;
                    }
                }
            } else {
                println!("Non-clickable element")
            }
        }
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if keycode == event::KeyCode::Escape {
            event::quit(ctx);
        } else if keycode == event::KeyCode::R {
            self.game = Game::new();
            self.history = Vec::new();
            self.viewing_history = false;
            self.selected_square = None;
            self.highlighted_squares = Vec::new();
        }
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("chess", "emil")
        .add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()
                .title("Chess") // Set window title "Chess"
                .icon("/icon.png"), // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false), // Fixate window size
        )
        .modules(conf::ModuleConf::default().audio(false));
    let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}

fn move_to_string(from: (usize, usize), to: (usize, usize)) -> String {
    let mut string = String::new();
    match from.1 {
        0 => string.push('a'),
        1 => string.push('b'),
        2 => string.push('c'),
        3 => string.push('d'),
        4 => string.push('e'),
        5 => string.push('f'),
        6 => string.push('g'),
        7 => string.push('h'),
        _ => panic!("No such file"),
    };
    match from.0 {
        0..=7 => string.push(char::from_digit(8 - from.0 as u32, 10).unwrap()),
        _ => panic!("No such rank"),
    };
    string.push(' ');
    match to.1 {
        0 => string.push('a'),
        1 => string.push('b'),
        2 => string.push('c'),
        3 => string.push('d'),
        4 => string.push('e'),
        5 => string.push('f'),
        6 => string.push('g'),
        7 => string.push('h'),
        _ => panic!("No such file"),
    };
    match to.0 {
        0..=7 => string.push(char::from_digit(8 - to.0 as u32, 10).unwrap()),
        _ => panic!("No such rank"),
    };
    string
}

fn get_colour(piece: Piece) -> Option<Colour> {
    match piece {
        Piece::King(c)
        | Piece::Queen(c)
        | Piece::Rook(c)
        | Piece::Knight(c)
        | Piece::Bishop(c)
        | Piece::Pawn(c) => Some(c),
        Piece::Empty => None,
    }
}
