use serde::{Deserialize, Serialize};
use crate::rules::{Board, Color, Position, Piece};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub captured_piece: Option<Piece>,
    // We'll add more fields later for special moves like castling and promotion
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Check { color: Color },
    Checkmate { winner: Color },
    Stalemate,
    Draw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub current_player: Color,
    pub move_history: Vec<Move>,
    pub status: GameStatus,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Board::new_game(),
            current_player: Color::White,
            move_history: Vec::new(),
            status: GameStatus::InProgress,
        }
    }

    pub fn make_move(&mut self, from: Position, to: Position) -> bool {
        // Basic move execution (to be expanded)
        if let Some(piece) = self.board.get_piece(from) {
            if piece.color != self.current_player {
                return false;
            }

            let captured_piece = self.board.get_piece(to);
            
            // Record the move
            let chess_move = Move {
                from,
                to,
                captured_piece,
            };

            // Execute the move
            self.board.set_piece(to, Some(piece));
            self.board.set_piece(from, None);

            // Add to history
            self.move_history.push(chess_move);

            // Switch current player
            self.current_player = self.current_player.opposite();

            // Update game status (to be implemented)
            self.update_game_status();

            true
        } else {
            false
        }
    }

    fn update_game_status(&mut self) {
        // This will be implemented later to check for:
        // - Check
        // - Checkmate
        // - Stalemate
        // - Draw conditions
    }

    pub fn undo_last_move(&mut self) -> bool {
        if let Some(last_move) = self.move_history.pop() {
            // Get the piece at the destination
            if let Some(piece) = self.board.get_piece(last_move.to) {
                // Move piece back
                self.board.set_piece(last_move.from, Some(piece));
                // Restore captured piece or set to None
                self.board.set_piece(last_move.to, last_move.captured_piece);
                // Switch back to previous player
                self.current_player = self.current_player.opposite();
                // Update game status
                self.update_game_status();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

