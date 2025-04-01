use std;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::{Color, Piece, PieceType, Position};

/// Represents a chess board.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Board {
    pieces: HashMap<Position, Piece>,
}

impl Board {
    /// Creates an empty chess board.
    pub fn new() -> Self {
        Board { pieces: HashMap::new() }
    }

    /// Creates a new chess board with assets in standard starting positions.
    pub fn new_game() -> Self {
        let mut board = Self::new();
        
        // Place pawns
        for file in 0..8 {
            board.set_piece(Position::new(file, 1), Piece::new(PieceType::Pawn, Color::White));
            board.set_piece(Position::new(file, 6), Piece::new(PieceType::Pawn, Color::Black));
        }
        
        // Place the assets on the back ranks
        let pieces = [
            PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen,
            PieceType::King, PieceType::Bishop, PieceType::Knight, PieceType::Rook
        ];
        
        for (file, &piece_type) in pieces.iter().enumerate() {
            board.set_piece(Position::new(file as u8, 0), Piece::new(piece_type, Color::White));
            board.set_piece(Position::new(file as u8, 7), Piece::new(piece_type, Color::Black));
        }
        
        board
    }

    /// Returns a reference to the piece at the given position, if any.
    pub fn get_piece(&self, pos: &Position) -> Option<&Piece> {
        self.pieces.get(pos)
    }

    /// Places a piece at the given position, replacing any existing piece.
    pub fn set_piece(&mut self, pos: Position, piece: Piece) {
        self.pieces.insert(pos, piece);
    }

    /// Removes and returns the piece at the given position, if any.
    pub fn remove_piece(&mut self, pos: &Position) -> Option<Piece> {
        self.pieces.remove(pos)
    }

    /// Returns true if the board has no assets.
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }

    /// Validates if a move is legal according to chess rules.
    pub fn is_valid_move(&self, from: &Position, to: &Position) -> bool {
        // Get piece at starting position
        let piece = match self.get_piece(from) {
            Some(p) => p,
            None => return false,
        };
        
        // Cannot move to a position occupied by own piece
        if let Some(target) = self.get_piece(to) {
            if target.color == piece.color {
                return false;
            }
        }
        
        match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(from, to, piece.color),
            PieceType::Knight => self.is_valid_knight_move(from, to),
            PieceType::Bishop => self.is_valid_bishop_move(from, to),
            PieceType::Rook => self.is_valid_rook_move(from, to),
            PieceType::Queen => self.is_valid_queen_move(from, to),
            PieceType::King => self.is_valid_king_move(from, to),
        }
    }

    // Add helper methods for basic move validation
    fn is_diagonal_move(&self, from: &Position, to: &Position) -> bool {
        let file_diff = (from.file as i16 - to.file as i16).abs();
        let rank_diff = (from.rank as i16 - to.rank as i16).abs();
        file_diff == rank_diff
    }

    fn is_straight_move(&self, from: &Position, to: &Position) -> bool {
        from.file == to.file || from.rank == to.rank
    }

    /// Attempts to make a move from one position to another.
    /// Returns true if the move was valid and executed, false otherwise.
    pub fn make_move(&mut self, from: &Position, to: &Position) -> bool {
        if !self.is_valid_move(from, to) {
            return false;
        }

        if let Some(piece) = self.remove_piece(from) {
            self.set_piece(*to, piece);
            true
        } else {
            false
        }
    }

    /// Returns all valid moves for a piece at the given position.
    pub fn get_valid_moves(&self, pos: &Position) -> Vec<Position> {
        let mut valid_moves = Vec::new();
        
        if let Some(_piece) = self.get_piece(pos) {
            // Check all possible destination squares
            for rank in 0..8 {
                for file in 0..8 {
                    let dest = Position::new(file, rank);
                    if self.is_valid_move(pos, &dest) {
                        valid_moves.push(dest);
                    }
                }
            }
        }
        
        valid_moves
    }
    
    // Piece-specific move validation methods
    fn is_valid_pawn_move(&self, from: &Position, to: &Position, color: Color) -> bool {
        // Implement pawn movement rules
        let direction = if color == Color::White { 1 } else { -1 };
        let file_diff = (to.file as i8 - from.file as i8).abs();
        let rank_diff = to.rank as i8 - from.rank as i8;
        
        // Pawns can move forward 1 square
        if file_diff == 0 && rank_diff == direction && self.get_piece(to).is_none() {
            return true;
        }
        
        // Pawns can move forward 2 squares from starting position
        let starting_rank = if color == Color::White { 1 } else { 6 };
        if file_diff == 0 && from.rank == starting_rank && rank_diff == 2 * direction {
            let intermediate = Position::new(from.file, (from.rank as i8 + direction) as u8);
            return self.get_piece(&intermediate).is_none() && self.get_piece(to).is_none();
        }
        
        // Pawns can capture diagonally
        if file_diff == 1 && rank_diff == direction && self.get_piece(to).is_some() {
            return true;
        }
        
        // TODO: Implement en passant and promotion
        
        false
    }
    
    fn is_valid_knight_move(&self, from: &Position, to: &Position) -> bool {
        let file_diff = (from.file as i8 - to.file as i8).abs();
        let rank_diff = (from.rank as i8 - to.rank as i8).abs();
        
        // Knights move in an L-shape pattern
        (file_diff == 1 && rank_diff == 2) || (file_diff == 2 && rank_diff == 1)
    }
    
    fn is_valid_bishop_move(&self, from: &Position, to: &Position) -> bool {
        if !self.is_diagonal_move(from, to) {
            return false;
        }
        
        // Check if path is clear
        self.is_path_clear(from, to)
    }
    
    fn is_valid_rook_move(&self, from: &Position, to: &Position) -> bool {
        if !self.is_straight_move(from, to) {
            return false;
        }
        
        // Check if path is clear
        self.is_path_clear(from, to)
    }
    
    fn is_valid_queen_move(&self, from: &Position, to: &Position) -> bool {
        // Queen combines rook and bishop movement
        (self.is_diagonal_move(from, to) || self.is_straight_move(from, to)) 
            && self.is_path_clear(from, to)
    }
    
    fn is_valid_king_move(&self, from: &Position, to: &Position) -> bool {
        let file_diff = (from.file as i8 - to.file as i8).abs();
        let rank_diff = (from.rank as i8 - to.rank as i8).abs();
        
        // King can move one square in any direction
        file_diff <= 1 && rank_diff <= 1
        
        // TODO: Implement castling
    }
    
    // Check if path between positions is clear of assets
    fn is_path_clear(&self, from: &Position, to: &Position) -> bool {
        let file_diff = to.file as i16 - from.file as i16;
        let rank_diff = to.rank as i16 - from.rank as i16;
        
        let file_step = file_diff.signum();
        let rank_step = rank_diff.signum();
        
        let mut file = from.file as i16 + file_step;
        let mut rank = from.rank as i16 + rank_step;
        
        while file != to.file as i16 || rank != to.rank as i16 {
            if self.get_piece(&Position::new(file as u8, rank as u8)).is_some() {
                return false;
            }
            
            file += file_step;
            rank += rank_step;
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Color, Piece, PieceType, Position};
    
    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        assert!(board.is_empty());
    }
    
    #[test]
    fn test_set_and_get_piece() {
        let mut board = Board::new();
        let pos = Position::new(3, 4);
        let piece = Piece::new(PieceType::Queen, Color::White);
        
        board.set_piece(pos, piece.clone());
        
        assert_eq!(board.get_piece(&pos), Some(&piece));
        assert!(!board.is_empty());
    }
    
    #[test]
    fn test_remove_piece() {
        let mut board = Board::new();
        let pos = Position::new(1, 1);
        let piece = Piece::new(PieceType::Pawn, Color::Black);
        
        board.set_piece(pos, piece.clone());
        let removed = board.remove_piece(&pos);
        
        assert_eq!(removed, Some(piece));
        assert_eq!(board.get_piece(&pos), None);
        assert!(board.is_empty());
    }
    
    #[test]
    fn test_new_game_has_32_pieces() {
        let board = Board::new_game();
        assert_eq!(board.pieces.len(), 32);
    }
    
    #[test]
    fn test_new_game_pawns_in_correct_positions() {
        let board = Board::new_game();
        
        // Check white pawns
        for file in 0..8 {
            let pos = Position::new(file, 1);
            let piece = board.get_piece(&pos).unwrap();
            assert_eq!(piece.piece_type, PieceType::Pawn);
            assert_eq!(piece.color, Color::White);
        }
        
        // Check black pawns
        for file in 0..8 {
            let pos = Position::new(file, 6);
            let piece = board.get_piece(&pos).unwrap();
            assert_eq!(piece.piece_type, PieceType::Pawn);
            assert_eq!(piece.color, Color::Black);
        }
    }
    
    #[test]
    fn test_new_game_major_pieces_in_correct_positions() {
        let board = Board::new_game();
        
        // Test piece layout for white assets
        assert_eq!(board.get_piece(&Position::new(0, 0)).unwrap().piece_type, PieceType::Rook);
        assert_eq!(board.get_piece(&Position::new(1, 0)).unwrap().piece_type, PieceType::Knight);
        assert_eq!(board.get_piece(&Position::new(2, 0)).unwrap().piece_type, PieceType::Bishop);
        assert_eq!(board.get_piece(&Position::new(3, 0)).unwrap().piece_type, PieceType::Queen);
        assert_eq!(board.get_piece(&Position::new(4, 0)).unwrap().piece_type, PieceType::King);
        assert_eq!(board.get_piece(&Position::new(5, 0)).unwrap().piece_type, PieceType::Bishop);
        assert_eq!(board.get_piece(&Position::new(6, 0)).unwrap().piece_type, PieceType::Knight);
        assert_eq!(board.get_piece(&Position::new(7, 0)).unwrap().piece_type, PieceType::Rook);
        
        // Test colors for white assets
        for file in 0..8 {
            assert_eq!(board.get_piece(&Position::new(file, 0)).unwrap().color, Color::White);
        }
        
        // Test piece layout for black assets
        assert_eq!(board.get_piece(&Position::new(0, 7)).unwrap().piece_type, PieceType::Rook);
        assert_eq!(board.get_piece(&Position::new(1, 7)).unwrap().piece_type, PieceType::Knight);
        assert_eq!(board.get_piece(&Position::new(2, 7)).unwrap().piece_type, PieceType::Bishop);
        assert_eq!(board.get_piece(&Position::new(3, 7)).unwrap().piece_type, PieceType::Queen);
        assert_eq!(board.get_piece(&Position::new(4, 7)).unwrap().piece_type, PieceType::King);
        assert_eq!(board.get_piece(&Position::new(5, 7)).unwrap().piece_type, PieceType::Bishop);
        assert_eq!(board.get_piece(&Position::new(6, 7)).unwrap().piece_type, PieceType::Knight);
        assert_eq!(board.get_piece(&Position::new(7, 7)).unwrap().piece_type, PieceType::Rook);
        
        // Test colors for black assets
        for file in 0..8 {
            assert_eq!(board.get_piece(&Position::new(file, 7)).unwrap().color, Color::Black);
        }
    }
    
    #[test]
    fn test_move_validation() {
        let mut board = Board::new_game();
        
        // Test pawn moves
        let e2 = Position::new(4, 1);
        let e3 = Position::new(4, 2);
        let e4 = Position::new(4, 3);
        
        // Valid single pawn move
        assert!(board.is_valid_move(&e2, &e3));
        
        // Valid double pawn move from starting position
        assert!(board.is_valid_move(&e2, &e4));
        
        // Invalid backward pawn move
        let backward = Position::new(4, 0);
        assert!(!board.is_valid_move(&e2, &backward));
        
        // Test knight moves
        let g1 = Position::new(6, 0);  // White knight starting position
        let f3 = Position::new(5, 2);
        let h3 = Position::new(7, 2);
        let e2 = Position::new(4, 1);
        
        // Valid knight moves
        assert!(board.is_valid_move(&g1, &f3));
        assert!(board.is_valid_move(&g1, &h3));
        
        // Invalid knight move
        assert!(!board.is_valid_move(&g1, &e2));
        
        // Test bishop move (need to clear path first)
        board.remove_piece(&Position::new(4, 1)); // Remove pawn blocking bishop
        let f1 = Position::new(5, 0);  // White bishop starting position
        let b5 = Position::new(1, 4);
        
        // Valid bishop move
        assert!(board.is_valid_move(&f1, &b5));
        
        // Test illegal move (blocked path)
        let blocked_pos = Position::new(3, 2);
        board.set_piece(blocked_pos, Piece::new(PieceType::Pawn, Color::White));
        assert!(!board.is_valid_move(&f1, &b5));
    }
    
    #[test]
    fn test_diagonal_and_straight_moves() {
        let board = Board::new();
        
        // Test diagonal moves
        let a1 = Position::new(0, 0);
        let h8 = Position::new(7, 7);
        assert!(board.is_diagonal_move(&a1, &h8));
        
        let e4 = Position::new(4, 3);
        let b7 = Position::new(1, 6);
        assert!(board.is_diagonal_move(&e4, &b7));
        
        // Non-diagonal move
        let a2 = Position::new(0, 1);
        assert!(!board.is_diagonal_move(&a1, &a2));
        
        // Test straight moves
        let a1 = Position::new(0, 0);
        let a8 = Position::new(0, 7);
        assert!(board.is_straight_move(&a1, &a8));
        
        let e4 = Position::new(4, 3);
        let h4 = Position::new(7, 3);
        assert!(board.is_straight_move(&e4, &h4));
        
        // Neither straight nor diagonal
        let b3 = Position::new(1, 2);
        assert!(!board.is_straight_move(&a1, &b3));
        assert!(!board.is_diagonal_move(&a1, &h4));
    }
    
    #[test]
    fn test_make_move() {
        let mut board = Board::new_game();
        
        // Test valid pawn move
        let e2 = Position::new(4, 1);
        let e4 = Position::new(4, 3);
        assert!(board.make_move(&e2, &e4));
        assert!(board.get_piece(&e2).is_none());
        assert!(board.get_piece(&e4).is_some());
        
        // Test invalid move
        let a7 = Position::new(0, 6);
        let a6 = Position::new(0, 5);
        assert!(!board.make_move(&a7, &a6));
    }
}

