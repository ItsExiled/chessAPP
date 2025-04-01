use iced::widget::{button, column, container, row, text, Column, Row, Container};
use iced::{Alignment, Element, Length, Color as IcedColor, Theme};
// Add these imports
use iced::theme;
use crate::types::{Position, Color};
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
                
                let piece_text = if let Some(piece) = game_state.board.get_piece(&pos) {
                    let mut txt = text(GameState::get_piece_symbol(piece))
                        .size(40);
                        
                    if piece.color == Color::Black {
                        txt = txt.style(IcedColor::BLACK);
                    }
                    
                    txt
                } else {
                    text("")
                };
                
                let square = Container::new(piece_text)
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
