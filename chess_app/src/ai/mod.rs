use crate::board::Board;
use crate::types::{Color, Position, PieceType};
use crate::state::GameState;

pub struct ChessAI {
    color: Color,
    depth: u8,
}

impl ChessAI {
    pub fn new(color: Color, difficulty: Difficulty) -> Self {
        let depth = match difficulty {
            Difficulty::Beginner => 2,
            Difficulty::Intermediate => 3,
            Difficulty::Advanced => 4,
        };
        
        ChessAI { color, depth }
    }
    
    pub fn get_best_move(&self, game_state: &GameState) -> Option<(Position, Position)> {
        let mut alpha = f32::NEG_INFINITY;
        let beta = f32::INFINITY;
        let mut best_move = None;
        let mut best_value = f32::NEG_INFINITY;
        
        // Get all possible moves
        let moves = self.generate_moves(&game_state.board, self.color);
        
        for (from, to) in moves {
            // Create a new board with the move applied
            let mut new_board = game_state.board.clone();
            if let Some(piece) = new_board.get_piece(&from) {
                new_board.set_piece(to, piece.clone());
                new_board.remove_piece(&from);
                
                // Calculate value using minimax
                let value = -self.minimax(&new_board, self.depth - 1, -beta, -alpha, self.color.opposite());
                
                if value > best_value {
                    best_value = value;
                    best_move = Some((from, to));
                }
                
                alpha = alpha.max(value);
            }
        }
        
        best_move
    }
    
    fn minimax(&self, board: &Board, depth: u8, mut alpha: f32, beta: f32, color: Color) -> f32 {
        if depth == 0 {
            return self.evaluate_position(board, color);
        }
        
        let moves = self.generate_moves(board, color);
        
        if moves.is_empty() {
            return self.evaluate_position(board, color);
        }
        
        let mut max_value = f32::NEG_INFINITY;
        
        for (from, to) in moves {
            let mut new_board = board.clone();
            if let Some(piece) = new_board.get_piece(&from) {
                new_board.set_piece(to, piece.clone());
                new_board.remove_piece(&from);
                
                let value = -self.minimax(&new_board, depth - 1, -beta, -alpha, color.opposite());
                max_value = max_value.max(value);
                alpha = alpha.max(value);
                
                if alpha >= beta {
                    break;
                }
            }
        }
        
        max_value
    }
    
    fn evaluate_position(&self, board: &Board, color: Color) -> f32 {
        let mut value = 0.0;
        
        // Simple material counting
        for rank in 0..8 {
            for file in 0..8 {
                let pos = Position::new(file, rank);
                if let Some(piece) = board.get_piece(&pos) {
                    let piece_value = match piece.piece_type {
                        PieceType::Pawn => 1.0,
                        PieceType::Knight => 3.0,
                        PieceType::Bishop => 3.0,
                        PieceType::Rook => 5.0,
                        PieceType::Queen => 9.0,
                        PieceType::King => 0.0, // King's value isn't counted
                    };
                    
                    if piece.color == color {
                        value += piece_value;
                    } else {
                        value -= piece_value;
                    }
                }
            }
        }
        
        value
    }
    
    fn generate_moves(&self, board: &Board, color: Color) -> Vec<(Position, Position)> {
        let mut moves = Vec::new();
        
        // Basic move generation (to be expanded)
        for rank in 0..8 {
            for file in 0..8 {
                let from = Position::new(file, rank);
                if let Some(piece) = board.get_piece(&from) {
                    if piece.color == color {
                        // Get valid moves for this piece
                        let valid_moves = board.get_valid_moves(&from);
                        for to in valid_moves {
                            moves.push((from, to));
                        }
                    }
                }
            }
        }
        
        moves
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}
