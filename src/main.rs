use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult};
use murnion_chess::{Colour, Game, Piece};
use std::path;

mod draw;

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
pub struct AppState {
    pub sprites: Vec<(Piece, graphics::Image)>,
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
            (Piece::Bishop(Colour::Black), "/black_bishop.png".to_string()),
            (Piece::Knight(Colour::Black), "/black_knight.png".to_string()),
            (Piece::King(Colour::White), "/white_king.png".to_string()),
            (Piece::Queen(Colour::White), "/white_queen.png".to_string()),
            (Piece::Rook(Colour::White), "/white_rook.png".to_string()),
            (Piece::Pawn(Colour::White), "/white_pawn.png".to_string()),
            (Piece::Bishop(Colour::White), "/white_bishop.png".to_string()),
            (Piece::Knight(Colour::White), "/white_knight.png".to_string()),
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

        draw::board(&self, ctx);
        draw::promotion_selector(&self, ctx);
        draw::history(&self, ctx);
        
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

        if self.viewing_history { // Move to function change to text on screen? Make text for if game over as well.
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
                    (0, 0) => 1,
                    (0, 1) => 2,
                    (1, 0) => 3,
                    (1, 1) => 4,
                    (2, 0) => 5,
                    (2, 1) => 6,
                    (3, 0) => 7,
                    (3, 1) => 8,
                    (4, 0) => 9,
                    (4, 1) => 10,
                    (5, 0) => 11,
                    (5, 1) => 12,
                    (6, 0) => 13,
                    (6, 1) => 14,
                    (7, 0) => 15,
                    (7, 1) => 16,
                    (8, 0) => 17,
                    (8, 1) => 18,
                    (9, 0) => 19,
                    (9, 1) => 20,
                    (10, 0) => 21,
                    (10, 1) => 22,
                    (11, 0) => 23,
                    (11, 1) => 24,
                    _ => panic!("Out of bounds!"),
                };

                if index <= self.history.len() && !self.viewing_history {
                    self.history.push(self.game.get_fen());
                    self.game.set_state_from_fen(&self.history[index - 1]);
                    self.viewing_history = true;
                } else if index < self.history.len() && self.viewing_history {
                    self.game.set_state_from_fen(&self.history[index - 1]);
                } else if index == self.history.len() && self.viewing_history {
                    if let Some(current_turn) = self.history.pop() {
                        self.game.set_state_from_fen(&current_turn);
                        self.viewing_history = false;
                    }
                }
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