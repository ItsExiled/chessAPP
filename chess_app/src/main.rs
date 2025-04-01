mod gui;
mod state;
mod board;
mod types;
// Comment out the rules module which duplicates types
// mod rules;
mod ai;

use ai::ChessAI;
use types::{Color, Position};
use iced::{
    executor, window, Application, Element, Settings, Theme,
    Command,
};

use gui::{GuiState, GuiMessage, Screen};
use state::GameState;

pub struct ChessApp {
    gui_state: GuiState,
    game_state: Option<GameState>,
    chess_ai: Option<ChessAI>,
}

impl Application for ChessApp {
    type Executor = executor::Default;
    type Message = GuiMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (ChessApp, Command<GuiMessage>) {
        (
            ChessApp {
                gui_state: GuiState::new(),
                game_state: None,
                chess_ai: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Chess Game")
    }

    fn update(&mut self, message: GuiMessage) -> Command<GuiMessage> {
        match message {
            GuiMessage::NewGame => {
                self.game_state = Some(GameState::new());
                self.gui_state.screen = Screen::Game;
                self.chess_ai = Some(ChessAI::new(Color::Black, self.gui_state.selected_difficulty.clone()));
            }
            GuiMessage::SetDifficulty(difficulty) => {
                self.gui_state.selected_difficulty = difficulty;
            }
            GuiMessage::LoadGame => {
                // TODO: Implement game loading
            }
            GuiMessage::BackToMenu => {
                self.gui_state.screen = Screen::MainMenu;
                self.game_state = None;
                self.chess_ai = None;
            }
            GuiMessage::SquareSelected(pos) => {
                // Handle square selection for moves
                if let Some(selected) = self.gui_state.selected_square {
                    // Attempt to make a move if a square was already selected
                    if let Some(game_state) = &mut self.game_state {
                        if game_state.board.is_valid_move(&selected, &pos) {
                            // Get the captured piece before making the move
                            let captured_piece = game_state.board.get_piece(&pos).cloned();
                            
                            // Make the move
                            game_state.board.make_move(&selected, &pos);
                            
                            // Captured piece is already handled by the make_move function
                            
                            // Record the move in game state
                            game_state.record_move(selected, pos, None);
                            
                            // Check if opponent's king is in check after the move
                            let opponent_color = game_state.current_player.opposite();
                            let is_check = game_state.board.is_king_in_check(opponent_color);
                            
                            // Switch turns
                            game_state.switch_turn();
                            
                            // Update game status based on check state
                            if is_check {
                                // Check if it's checkmate by seeing if opponent has any valid moves
                                let is_checkmate = Self::is_checkmate(&game_state.board, opponent_color);
                                
                                if is_checkmate {
                                    game_state.status = state::GameStatus::Checkmate { 
                                        winner: opponent_color.opposite() 
                                    };
                                } else {
                                    game_state.status = state::GameStatus::Check { 
                                        player: opponent_color 
                                    };
                                }
                            } else {
                                // Check for stalemate
                                let is_stalemate = Self::is_stalemate(&game_state.board, opponent_color);
                                if is_stalemate {
                                    game_state.status = state::GameStatus::Stalemate;
                                } else {
                                    game_state.status = state::GameStatus::InProgress;
                                }
                            }
                            
                            // If it's now the AI's turn (Black), make an AI move
                            if game_state.current_player == Color::Black && 
                               game_state.status == state::GameStatus::InProgress ||
                               matches!(game_state.status, state::GameStatus::Check { player: Color::Black }) {
                                if let Some(chess_ai) = &self.chess_ai {
                                    if let Some((from, to)) = chess_ai.get_best_move(game_state) {
                                        // Make the AI's move
                                        if game_state.board.is_valid_move(&from, &to) {
                                            // Get the captured piece before making the move
                                            let captured_piece = game_state.board.get_piece(&to).cloned();
                                            
                                            // Make the move
                                            game_state.board.make_move(&from, &to);
                                            
                                            // Captured piece is already handled by the make_move function
                                            
                                            // Record the move in game state
                                            game_state.record_move(from, to, None);
                                            
                                            // Check if opponent's king is in check after the move
                                            let opponent_color = game_state.current_player.opposite();
                                            let is_check = game_state.board.is_king_in_check(opponent_color);
                                            
                                            // Switch back to player's turn
                                            game_state.switch_turn();
                                            
                                            // Update game status based on check state
                                            if is_check {
                                                // Check if it's checkmate by seeing if opponent has any valid moves
                                                let is_checkmate = Self::is_checkmate(&game_state.board, opponent_color);
                                                
                                                if is_checkmate {
                                                    game_state.status = state::GameStatus::Checkmate { 
                                                        winner: opponent_color.opposite() 
                                                    };
                                                } else {
                                                    game_state.status = state::GameStatus::Check { 
                                                        player: opponent_color 
                                                    };
                                                }
                                            } else {
                                                // Check for stalemate
                                                let is_stalemate = Self::is_stalemate(&game_state.board, opponent_color);
                                                if is_stalemate {
                                                    game_state.status = state::GameStatus::Stalemate;
                                                } else {
                                                    game_state.status = state::GameStatus::InProgress;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self.gui_state.selected_square = None;
                } else {
                    // Select the square if it contains a piece of the current player
                    if let Some(game_state) = &self.game_state {
                        if let Some(piece) = game_state.board.get_piece(&pos) {
                            if piece.color == game_state.current_player {
                                self.gui_state.selected_square = Some(pos);
                            }
                        }
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<GuiMessage> {
        self.gui_state.view(self.game_state.as_ref())
    }
}

impl ChessApp {
    /// Check if a player is in checkmate (static function)
    fn is_checkmate(board: &board::Board, player_color: Color) -> bool {
        // If the king is not in check, it's not checkmate
        if !board.is_king_in_check(player_color) {
            return false;
        }
        
        // Check if any move can get the king out of check
        for from_rank in 0..8 {
            for from_file in 0..8 {
                let from = Position::new(from_file, from_rank);
                
                if let Some(piece) = board.get_piece(&from) {
                    if piece.color == player_color {
                        // Try all possible destination squares
                        for to_rank in 0..8 {
                            for to_file in 0..8 {
                                let to = Position::new(to_file, to_rank);
                                
                                // If we find a valid move, not checkmate
                                if board.is_valid_move(&from, &to) {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // If no valid moves found, it's checkmate
        true
    }
    
    /// Check if a player is in stalemate (static function)
    fn is_stalemate(board: &board::Board, player_color: Color) -> bool {
        // If the king is in check, it's not stalemate
        if board.is_king_in_check(player_color) {
            return false;
        }
        
        // Check if the player has any valid moves
        for from_rank in 0..8 {
            for from_file in 0..8 {
                let from = Position::new(from_file, from_rank);
                
                if let Some(piece) = board.get_piece(&from) {
                    if piece.color == player_color {
                        // Try all possible destination squares
                        for to_rank in 0..8 {
                            for to_file in 0..8 {
                                let to = Position::new(to_file, to_rank);
                                
                                // If we find a valid move, not stalemate
                                if board.is_valid_move(&from, &to) {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // If no valid moves found, it's stalemate
        true
    }
}

pub fn main() -> iced::Result {
    // Create assets directory if it doesn't exist
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/home/exiled"));
    let assets_dir = format!("{}/chessAPP/chess_app/assets", home);
    std::fs::create_dir_all(&assets_dir).ok();
    
    ChessApp::run(Settings {
        window: window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        antialiasing: true,  // Enable antialiasing for better text rendering
        ..Default::default()
    })
}
