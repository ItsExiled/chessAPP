use iced::widget::{button, column, container, row, text, Column, Row, Container, image};
use iced::{Alignment, Element, Length, Color as IcedColor, Theme};
// Add these imports
use iced::theme;
use iced::widget::image::Handle;
use std::path::PathBuf;
use resvg::usvg::{Tree, Options, TreeParsing};
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::FitTo;
use std::fs;
use crate::types::{Position, Color, PieceType, Piece};
use crate::state::GameState;

#[derive(Debug, Clone)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone)]
pub enum Screen {
    MainMenu,
    Game,
}

#[derive(Debug, Clone)]
pub struct GuiState {
    pub screen: Screen,
    pub selected_difficulty: Difficulty,
    pub selected_square: Option<Position>,
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    NewGame,
    SetDifficulty(Difficulty),
    LoadGame,
    BackToMenu,
    SquareSelected(Position),
}

// Create a custom style for chess squares
#[derive(Debug, Clone, Copy)]
pub struct ChessSquareStyle {
    is_dark: bool,
    is_selected: bool,
}

impl container::StyleSheet for ChessSquareStyle {
    type Style = Theme;

    fn appearance(&self, _theme: &Self::Style) -> container::Appearance {
        let background = if self.is_selected {
            IcedColor::from_rgb(0.7, 0.7, 1.0)
        } else if self.is_dark {
            IcedColor::from_rgb(0.6, 0.4, 0.2)
        } else {
            IcedColor::from_rgb(1.0, 0.9, 0.7)
        };

        container::Appearance {
            background: Some(background.into()),
            ..Default::default()
        }
    }
}

// Fix the implementation to work with custom styling
impl From<ChessSquareStyle> for theme::Container {
    fn from(style: ChessSquareStyle) -> Self {
        theme::Container::Custom(Box::new(style))
    }
}

// Helper function to get piece asset path
fn get_piece_asset_path(piece: &Piece) -> PathBuf {
    let color_str = match piece.color {
        Color::White => "l", // l for light (white)
        Color::Black => "d", // d for dark (black)
    };
    
    let piece_str = match piece.piece_type {
        PieceType::King => "k",   // king
        PieceType::Queen => "q",  // queen
        PieceType::Rook => "r",   // rook
        PieceType::Bishop => "b", // bishop
        PieceType::Knight => "n", // knight
        PieceType::Pawn => "p",   // pawn
    };
    
    let filename = format!("Chess_{}{}{}.svg", piece_str, color_str, "t45");
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/home/exiled"));
    PathBuf::from(format!("{}/chessAPP/chess_app/assets/{}", home, filename))
}

fn load_svg(path: &PathBuf, width: u32, height: u32) -> Option<Handle> {
    // Read SVG file
    let svg_data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading SVG file: {}, error: {}", path.display(), e);
            return None;
        }
    };
    
    // Parse SVG
    let opt = Options::default();
    let tree = match Tree::from_str(&svg_data, &opt) {
        Ok(tree) => tree,
        Err(e) => {
            println!("Error parsing SVG: {}, error: {}", path.display(), e);
            return None;
        }
    };
    
    // Create a pixmap to render to
    let mut pixmap = match Pixmap::new(width, height) {
        Some(pixmap) => pixmap,
        None => {
            println!("Error creating pixmap for {}", path.display());
            return None;
        }
    };
    
    // Render SVG to pixmap
    resvg::render(&tree, FitTo::Size(width, height), Transform::default(), pixmap.as_mut());
    
    // Convert to RGBA bytes
    let rgba = pixmap.take();
    
    // Create image handle
    Some(Handle::from_pixels(width, height, rgba))
}

// Add this function for simpler fallback piece representation
fn get_simple_piece_text(piece: &Piece) -> String {
    let color_char = match piece.color {
        Color::White => "W",
        Color::Black => "B",
    };
    
    let piece_char = match piece.piece_type {
        PieceType::King => "K",
        PieceType::Queen => "Q",
        PieceType::Rook => "R",
        PieceType::Bishop => "B",
        PieceType::Knight => "N",
        PieceType::Pawn => "P",
    };
    
    format!("{}{}", color_char, piece_char)
}

impl GuiState {
    pub fn new() -> Self {
        GuiState {
            screen: Screen::MainMenu,
            selected_difficulty: Difficulty::Beginner,
            selected_square: None,
        }
    }

    pub fn view(&self, game_state: Option<&GameState>) -> Element<GuiMessage> {
        match self.screen {
            Screen::MainMenu => self.view_main_menu(),
            Screen::Game => {
                if let Some(game_state) = game_state {
                    self.view_game(game_state)
                } else {
                    // Fallback if game state is missing
                    container(text("Error: Game state missing"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y()
                        .into()
                }
            }
        }
    }

    fn view_main_menu(&self) -> Element<GuiMessage> {
        let title = text("Chess Game")
            .size(40)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let new_game_button = button("New Game")
            .width(Length::Fixed(200.0))
            .on_press(GuiMessage::NewGame);

        let load_game_button = button("Load Game")
            .width(Length::Fixed(200.0))
            .on_press(GuiMessage::LoadGame);

        let difficulty_row = row![
            button("Beginner")
                .on_press(GuiMessage::SetDifficulty(Difficulty::Beginner)),
            button("Intermediate")
                .on_press(GuiMessage::SetDifficulty(Difficulty::Intermediate)),
            button("Advanced")
                .on_press(GuiMessage::SetDifficulty(Difficulty::Advanced)),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let content = column![
            title,
            new_game_button,
            load_game_button,
            text("Select Difficulty:").size(20),
            difficulty_row,
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn view_game(&self, game_state: &GameState) -> Element<GuiMessage> {
        let mut board_container = Column::new().spacing(0);
        
        // Create the board rows
        for rank in (0..8).rev() {
            let mut board_row = Row::new().spacing(0);
            
            for file in 0..8 {
                let pos = Position::new(file, rank);
                let is_dark = (rank + file) % 2 == 1;
                let is_selected = self.selected_square == Some(pos);
                
                // Create a proper style struct
                let square_style = ChessSquareStyle {
                    is_dark,
                    is_selected,
                };
                
                // Use image widget instead of text for pieces
                let square_content: Element<_> = if let Some(piece) = game_state.board.get_piece(&pos) {
                    // Try to load the image asset
                    let asset_path = get_piece_asset_path(piece);
                    
                    // Debug prints to help diagnose asset loading issues
                    println!("Looking for asset: {}", asset_path.display());
                    println!("File exists: {}", asset_path.exists());
                    
                    if asset_path.exists() {
                        // If asset exists, use the image
                        let img = match load_svg(&asset_path, 50, 50) {
                            Some(handle) => handle,
                            None => {
                                println!("Failed to load SVG: {}", asset_path.display());
                                // Fallback to text representation
                                let symbol = GameState::get_piece_symbol(piece);
                                let piece_text = if symbol.starts_with('�') {
                                    get_simple_piece_text(piece)
                                } else {
                                    symbol.to_string()
                                };
                                
                                return text(piece_text).size(40).into();
                            }
                        };
                        image(img)
                            .width(Length::Fixed(50.0))
                            .height(Length::Fixed(50.0))
                            .into()
                    } else {
                        // First try Unicode symbol
                        let symbol = GameState::get_piece_symbol(piece);
                        // If symbol starts with �, it means Unicode failed, use simple text instead
                        let piece_text = if symbol.starts_with('�') {
                            get_simple_piece_text(piece)
                        } else {
                            symbol.to_string()
                        };
                        
                        let mut txt = text(piece_text).size(40);
                            
                        if piece.color == Color::Black {
                            txt = txt.style(IcedColor::BLACK);
                        }
                        
                        txt.into()
                    }
                } else {
                    text("").into()
                };
                
                let square = Container::new(square_content)
                    .width(Length::Fixed(60.0))
                    .height(Length::Fixed(60.0))
                    .style(square_style)
                    .center_x()
                    .center_y();
                
                let square_button = button(square)
                    .on_press(GuiMessage::SquareSelected(pos))
                    .padding(0);
                
                board_row = board_row.push(square_button);
            }
            
            board_container = board_container.push(board_row);
        }
        
        let back_button = button("Back to Menu")
            .on_press(GuiMessage::BackToMenu);
        
        column![
            board_container,
            back_button,
        ]
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }
}
