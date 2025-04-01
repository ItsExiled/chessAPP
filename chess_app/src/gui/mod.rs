use iced::{
    Element, Length, Size,
    widget::{container, row, column, button},
    theme, Background, Color,
};
use crate::{
    state::GameState,
    rules::{Position, Color as PieceColor},
    assets::PieceSprite,
};

#[derive(Debug, Clone)]
pub enum Message {
    SquareClicked(Position),
}

pub struct ChessGui {
    game_state: GameState,
    selected_square: Option<Position>,
}

impl ChessGui {
    pub fn new() -> Self {
        ChessGui {
            game_state: GameState::new(),
            selected_square: None,
        }
    }
    
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SquareClicked(pos) => {
                if let Some(selected) = self.selected_square {
                    // Try to move the piece
                    if self.game_state.board.is_legal_move(
                        selected,
                        pos,
                        self.game_state.current_player,
                        &self.game_state
                    ) {
                        // Move the piece
                        let piece = self.game_state.board.get_piece(selected).unwrap();
                        self.game_state.board.set_piece(pos, Some(piece));
                        self.game_state.board.set_piece(selected, None);
                        
                        // Record the move and switch turns
                        self.game_state.record_move(selected, pos, None);
                        self.game_state.switch_turn();
                    }
                    self.selected_square = None;
                } else {
                    // Select the piece if it belongs to the current player
                    if let Some(piece) = self.game_state.board.get_piece(pos) {
                        if piece.color == self.game_state.current_player {
                            self.selected_square = Some(pos);
                        }
                    }
                }
            }
        }
    }
    
    pub fn view(&self) -> Element<Message> {
        let mut board = column![].spacing(1).width(Length::Fill);
        
        for rank in (0..8).rev() {
            let mut row_widgets = row![].spacing(1);
            
            for file in 0..8 {
                let pos = Position { rank, file };
                let is_selected = self.selected_square == Some(pos);
                let is_dark = (rank + file) % 2 == 1;
                
                let square = self.create_square(pos, is_dark, is_selected);
                row_widgets = row_widgets.push(square);
            }
            
            board = board.push(row_widgets);
        }
        
        container(board)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
    
    fn create_square(&self, pos: Position, is_dark: bool, is_selected: bool) -> Element<Message> {
        let mut square = button(if let Some(piece) = self.game_state.board.get_piece(pos) {
            PieceSprite::widget(&piece)
        } else {
            container("").into()
        });
        
        let bg_color = if is_selected {
            Color::from_rgb(0.0, 0.5, 1.0)
        } else if is_dark {
            Color::from_rgb(0.5, 0.3, 0.1)
        } else {
            Color::from_rgb(1.0, 0.9, 0.7)
        };
        
        square = square
            .width(Length::Fixed(60.0))
            .height(Length::Fixed(60.0))
            .style(theme::Button::Custom(Box::new(move |_state| {
                theme::Button {
                    background: Some(Background::Color(bg_color)),
                    ..Default::default()
                }
            })));
        
        square.on_press(Message::SquareClicked(pos)).into()
    }
}

use iced::{
    widget::{container, row, column, svg, button, text},
    Element, Length, Color as IcedColor, Rectangle,
    mouse::Cursor,
    theme::Theme,
};
use crate::{
    rules::{Board, Position, Piece, Color},
    state::GameState,
    assets::PieceSprite,
};

pub struct ChessGui {
    game_state: GameState,
    selected_square: Option<Position>,
    valid_moves: Vec<Position>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SquareClicked(Position),
}

impl ChessGui {
    pub fn new() -> Self {
        ChessGui {
            game_state: GameState::new(),
            selected_square: None,
            valid_moves: Vec::new(),
        }
    }
    
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SquareClicked(pos) => {
                if let Some(selected) = self.selected_square {
                    // If a square was already selected, try to make a move
                    if self.valid_moves.contains(&pos) {
                        // Execute the move
                        let piece = self.game_state.board.get_piece(selected).unwrap();
                        self.game_state.board.set_piece(pos, Some(piece));
                        self.game_state.board.set_piece(selected, None);
                        
                        // Record the move
                        self.game_state.record_move(selected, pos, None);
                        
                        // Switch turns
                        self.game_state.switch_turn();
                    }
                    
                    // Clear selection
                    self.selected_square = None;
                    self.valid_moves.clear();
                } else {
                    // If no square was selected, select the clicked square if it has a piece
                    if let Some(piece) = self.game_state.board.get_piece(pos) {
                        if piece.color == self.game_state.current_player {
                            self.selected_square = Some(pos);
                            // Calculate valid moves for the selected piece
                            self.valid_moves = self.calculate_valid_moves(pos);
                        }
                    }
                }
            }
        }
    }
    
    fn calculate_valid_moves(&self, pos: Position) -> Vec<Position> {
        let mut valid_moves = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                let to = Position { rank, file };
                if self.game_state.board.is_valid_move(pos, to, self.game_state.current_player, &self.game_state) {
                    valid_moves.push(to);
                }
            }
        }
        valid_moves
    }
    
    pub fn view(&self) -> Element<Message> {
        let board = self.create_board_view();
        
        container(board)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    
    fn create_board_view(&self) -> Element<Message> {
        let mut board_column = column![];
        
        for rank in (0..8).rev() {
            let mut row_content = row![];
            
            for file in 0..8 {
                let pos = Position { rank, file };
                let square = self.create_square_view(pos);
                row_content = row_content.push(square);
            }
            
            board_column = board_column.push(row_content);
        }
        
        container(board_column)
            .style(theme::Container::Custom(Box::new(BoardStyle)))
            .into()
    }
    
    fn create_square_view(&self, pos: Position) -> Element<Message> {
        let is_dark = (pos.rank + pos.file) % 2 == 1;
        let is_selected = self.selected_square == Some(pos);
        let is_valid_move = self.valid_moves.contains(&pos);
        
        let mut square_content = column![];
        
        // Add piece sprite if there's a piece on this square
        if let Some(piece) = self.game_state.board.get_piece(pos) {
            square_content = square_content.push(
                container(PieceSprite::widget(&piece))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
            );
        }
        
        button(square_content)
            .width(Length::Fixed(60.0))
            .height(Length::Fixed(60.0))
            .style(theme::Button::Custom(Box::new(SquareStyle {
                is_dark,
                is_selected,
                is_valid_move,
            })))
            .on_press(Message::SquareClicked(pos))
            .into()
    }
}

struct SquareStyle {
    is_dark: bool,
    is_selected: bool,
    is_valid_move: bool,
}

impl iced::widget::button::StyleSheet for SquareStyle {
    type Style = Theme;
    
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let background = if self.is_dark {
            IcedColor::from_rgb(0.5, 0.5, 0.5)
        } else {
            IcedColor::from_rgb(0.9, 0.9, 0.9)
        };
        
        let border = if self.is_selected {
            Some((2.0, IcedColor::from_rgb(0.0, 0.7, 0.0)))
        } else if self.is_valid_move {
            Some((2.0, IcedColor::from_rgb(0.0, 0.0, 0.7)))
        } else {
            None
        };
        
        button::Appearance {
            background: Some(background.into()),
            border,
            ..Default::default()
        }
    }
}

struct BoardStyle;

impl container::StyleSheet for BoardStyle {
    type Style = Theme;
    
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_color: IcedColor::BLACK,
            border_width: 2.0,
            ..Default::default()
        }
    }
}

