use iced::widget::{button, column, container, row, text, Column, Row, Container};
use iced::{Alignment, Element, Length, Color as IcedColor};
use crate::rules::{Position, Piece, PieceType, Color};
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

impl GuiState {
    pub fn new() -> Self {
        GuiState {
            screen: Screen::MainMenu,
            selected_difficulty: Difficulty::Beginner,
            selected_square: None,
        }
    }
    
    fn get_piece_symbol(piece: &Piece) -> &'static str {
        match (piece.piece_type, piece.color) {
            (PieceType::King, Color::White) => "♔",
            (PieceType::Queen, Color::White) => "♕",
            (PieceType::Rook, Color::White) => "♖",
            (PieceType::Bishop, Color::White) => "♗",
            (PieceType::Knight, Color::White) => "♘",
            (PieceType::Pawn, Color::White) => "♙",
            (PieceType::King, Color::Black) => "♚",
            (PieceType::Queen, Color::Black) => "♛",
            (PieceType::Rook, Color::Black) => "♜",
            (PieceType::Bishop, Color::Black) => "♝",
            (PieceType::Knight, Color::Black) => "♞",
            (PieceType::Pawn, Color::Black) => "♟",
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
                let pos = Position { rank, file };
                let is_dark = (rank + file) % 2 == 1;
                let is_selected = self.selected_square == Some(pos);
                
                let square_color = if is_selected {
                    IcedColor::from_rgb(0.7, 0.7, 1.0)
                } else if is_dark {
                    IcedColor::from_rgb(0.6, 0.4, 0.2)
                } else {
                    IcedColor::from_rgb(1.0, 0.9, 0.7)
                };
                
                let piece_text = if let Some(piece) = game_state.board.get_piece(pos) {
                    text(Self::get_piece_symbol(&piece))
                        .size(40)
                        .style(if piece.color == Color::White {
                            iced::theme::Text::Default
                        } else {
                            iced::theme::Text::Color(IcedColor::BLACK)
                        })
                } else {
                    text("")
                };
                
                let square = Container::new(piece_text)
                    .width(Length::Fixed(60.0))
                    .height(Length::Fixed(60.0))
                    .style(iced::theme::Container::Custom(Box::new(move |_| {
                        iced::widget::container::Appearance {
                            background: Some(square_color.into()),
                            ..Default::default()
                        }
                    })))
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

