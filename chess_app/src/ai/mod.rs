use crate::rules::{Board, Color, Position, Piece, PieceType};
use crate::state::{GameState, Move};

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
            let piece = new_board.get_piece(from)?;
            new_board.set_piece(to, Some(piece));
            new_board.set_piece(from, None);
            
            // Calculate value using minimax
            let value = -self.minimax(&new_board, self.depth - 1, -beta, -alpha, self.color.opposite());
            
            if value > best_value {
                best_value = value;
                best_move = Some((from, to));
            }
            
            alpha = alpha.max(value);
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
            if let Some(piece) = new_board.get_piece(from) {
                new_board.set_piece(to, Some(piece));
                new_board.set_piece(from, None);
                
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
                if let Some(piece) = board.get_piece(Position { rank, file }) {
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
                let from = Position { rank, file };
                if let Some(piece) = board.get_piece(from) {
                    if piece.color == color {
                        // Add potential moves based on piece type
                        // (This is a placeholder - full implementation needed)
                        // For now, just add adjacent squares as possible moves
                        for &(dr, df) in &[(1,0), (-1,0), (0,1), (0,-1)] {
                            let new_rank = rank as i8 + dr;
                            let new_file = file as i8 + df;
                            
                            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                                let to = Position { 
                                    rank: new_rank as i8, 
                                    file: new_file as i8 
                                };
                                moves.push((from, to));
                            }
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

