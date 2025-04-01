use std::collections::HashMap;
use crate::types::{Color, Piece, PieceType, Position};

/// Represents a chess board.
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pieces: HashMap<Position, Piece>,
}

impl Board {
    /// Creates an empty chess board.
    pub fn new() -> Self {
        Board { pieces: HashMap::new() }
    }

    /// Creates a new chess board with pieces in standard starting positions.
    pub fn new_game() -> Self {
        let mut board = Self::new();
        
        // Place pawns
        for file in 0..8 {
            board.set_piece(Position::new(file, 1), Piece::new(PieceType::Pawn, Color::White));
            board.set_piece(Position::new(file, 6), Piece::new(PieceType::Pawn, Color::Black));
        }
        
        // Place the pieces on the back ranks
        let pieces = [
            PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen,
            PieceType::King, PieceType::Bishop, PieceType::Knight, PieceType::Rook
        ];
        
        for (file, &piece_type) in pieces.iter().enumerate() {
            board.set_piece(Position::new(file, 0), Piece::new(piece_type, Color::White));
            board.set_piece(Position::new(file, 7), Piece::new(piece_type, Color::Black));
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

    /// Returns true if the board has no pieces.
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
        let file_diff = (from.file as i8 - to.file as i8).abs();
        let rank_diff = (from.rank as i8 - to.rank as i8).abs();
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
        
        if let Some(piece) = self.get_piece(pos) {
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
        
        board.set_piece(pos.clone(), piece.clone());
        
        assert_eq!(board.get_piece(&pos), Some(&piece));
        assert!(!board.is_empty());
    }
    
    #[test]
    fn test_remove_piece() {
        let mut board = Board::new();
        let pos = Position::new(1, 1);
        let piece = Piece::new(PieceType::Pawn, Color::Black);
        
        board.set_piece(pos.clone(), piece.clone());
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
        
        // Test piece layout for white pieces
        assert_eq!(board.get_piece(&Position::new(0, 0)).unwrap().piece_type, PieceType::Rook);
        assert_eq!(board.get_piece(&Position::new(1, 0)).unwrap().piece_type, PieceType::Knight);
        assert_eq!(board.get_piece(&Position::new(2, 0)).unwrap().piece_type, PieceType::Bishop);
        assert_eq!(board.get_piece(&Position::new(3, 0)).unwrap().piece_type, PieceType::Queen);
        assert_eq!(board.get_piece(&Position::new(4, 0)).unwrap().piece_type, PieceType::King);
        assert_eq!(board.get_piece(&Position::new(5, 0)).unwrap().piece_type, PieceType::Bishop);
        assert_eq!(board.get_piece(&Position::new(6, 0)).unwrap().piece_type, PieceType::Knight);
        assert_eq!(board.get_piece(&Position::new(7, 0)).unwrap().piece_type, PieceType::Rook);
        
        // Test colors for white pieces
        for file in 0..8 {
            assert_eq!(board.get_piece(&Position::new(file, 0)).unwrap().color, Color::White);
        }
        
        // Test piece layout for black pieces
        assert_eq!(board.get_piece(&Position::new(0, 7)).unwrap().piece_type, PieceType::Rook);
        assert_eq!(board.get_piece(&Position::new(1, 7)).unwrap().piece_type, PieceType::Knight);
        assert_eq!(board.get_piece(&Position::new(2, 7)).unwrap().piece_type, PieceType::Bishop);
        assert_eq!(board.get_piece(&Position::new(3, 7)).unwrap().piece_type, PieceType::Queen);
        assert_eq!(board.get_piece(&Position::new(4, 7)).unwrap().piece_type, PieceType::King);
        assert_eq!(board.get_piece(&Position::new(5, 7)).unwrap().piece_type, PieceType::Bishop);
        assert_eq!(board.get_piece(&Position::new(6, 7)).unwrap().piece_type, PieceType::Knight);
        assert_eq!(board.get_piece(&Position::new(7, 7)).unwrap().piece_type, PieceType::Rook);
        
        // Test colors for black pieces
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
307|}

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
        assert!(!board.make_move(&a7, &a6)); // Black pawn can't move yet
    }

    #[test]
    fn test_get_valid_moves() {
        let mut board = Board::new_game();
        
        // Test pawn moves
        let e2 = Position::new(4, 1);
        let moves = board.get_valid_moves(&e2);
        assert_eq!(moves.len(), 2); // Can move one or two squares forward
        
        // Test knight moves
        let g1 = Position::new(6, 0);
        let moves = board.get_valid_moves(&g1);
        assert_eq!(moves.len(), 2); // Can move to f3 and h3
        
        // Test blocked piece
        let e1 = Position::new(4, 0); // White king, should be blocked
        let moves = board.get_valid_moves(&e1);
        assert_eq!(moves.len(), 0); // No valid moves initially
    }

    #[test]
    fn test_capture_moves() {
        let mut board = Board::new();
        
        // Set up a capture scenario
        let e4 = Position::new(4, 3);
        let d5 = Position::new(3, 4);
        
        board.set_piece(e4, Piece::new(PieceType::Pawn, Color::White));
        board.set_piece(d5, Piece::new(PieceType::Pawn, Color::Black));
        
        // White pawn should be able to capture black pawn
        assert!(board.is_valid_move(&e4, &d5));
        assert!(board.make_move(&e4, &d5));
        
        // Verify capture
        let captured = board.get_piece(&d5).unwrap();
        assert_eq!(captured.color, Color::White);
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
    
    // Check if path between positions is clear of pieces
    fn is_path_clear(&self, from: &Position, to: &Position) -> bool {
        let file_diff = to.file as i8 - from.file as i8;
        let rank_diff = to.rank as i8 - from.rank as i8;
        
        let file_step = file_diff.signum();
        let rank_step = rank_diff.signum();
        
        let mut file = from.file as i8 + file_step;
        let mut rank = from.rank as i8 + rank_step;
        
        while file != to.file as i8 || rank != to.rank as i8 {
            if self.get_piece(&Position::new(file as u8, rank as u8)).is_some() {
                return false;
            }
            
            file += file_step;
            rank += rank_step;
        }
        
        true
    }

//! Chess board implementation.

use std::collections::HashMap;
use crate::types::{Color, Piece, PieceType, Position};

/// Represents a chess board.
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pieces: HashMap<Position, Piece>,
}

impl Board {
    /// Creates a new empty chess board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    ///
    /// let board = Board::new();
    /// assert!(board.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            pieces: HashMap::new(),
        }
    }

    /// Creates a new chess board with pieces in starting positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, PieceType, Position};
    ///
    /// let board = Board::new_game();
    /// 
    /// // Verify white pawn at e2
    /// let e2 = Position::new(4, 1);
    /// let piece = board.get_piece(&e2).unwrap();
    /// assert_eq!(piece.piece_type, PieceType::Pawn);
    /// assert_eq!(piece.color, Color::White);
    /// ```
    pub fn new_game() -> Self {
        let mut board = Self::new();
        
        // Set up pawns
        for file in 0..8 {
            // White pawns on rank 1
            board.set_piece(
                Position::new(file, 1),
                Piece {
                    color: Color::White,
                    piece_type: PieceType::Pawn,
                },
            );
            
            // Black pawns on rank 6
            board.set_piece(
                Position::new(file, 6),
                Piece {
                    color: Color::Black,
                    piece_type: PieceType::Pawn,
                },
            );
        }
        
        // Set up other pieces
        // White pieces on rank 0
        board.set_piece(Position::new(0, 0), Piece { color: Color::White, piece_type: PieceType::Rook });
        board.set_piece(Position::new(1, 0), Piece { color: Color::White, piece_type: PieceType::Knight });
        board.set_piece(Position::new(2, 0), Piece { color: Color::White, piece_type: PieceType::Bishop });
        board.set_piece(Position::new(3, 0), Piece { color: Color::White, piece_type: PieceType::Queen });
        board.set_piece(Position::new(4, 0), Piece { color: Color::White, piece_type: PieceType::King });
        board.set_piece(Position::new(5, 0), Piece { color: Color::White, piece_type: PieceType::Bishop });
        board.set_piece(Position::new(6, 0), Piece { color: Color::White, piece_type: PieceType::Knight });
        board.set_piece(Position::new(7, 0), Piece { color: Color::White, piece_type: PieceType::Rook });
        
        // Black pieces on rank 7
        board.set_piece(Position::new(0, 7), Piece { color: Color::Black, piece_type: PieceType::Rook });
        board.set_piece(Position::new(1, 7), Piece { color: Color::Black, piece_type: PieceType::Knight });
        board.set_piece(Position::new(2, 7), Piece { color: Color::Black, piece_type: PieceType::Bishop });
        board.set_piece(Position::new(3, 7), Piece { color: Color::Black, piece_type: PieceType::Queen });
        board.set_piece(Position::new(4, 7), Piece { color: Color::Black, piece_type: PieceType::King });
        board.set_piece(Position::new(5, 7), Piece { color: Color::Black, piece_type: PieceType::Bishop });
        board.set_piece(Position::new(6, 7), Piece { color: Color::Black, piece_type: PieceType::Knight });
        board.set_piece(Position::new(7, 7), Piece { color: Color::Black, piece_type: PieceType::Rook });
        
        board
    }
    
    /// Gets a piece at the specified position if one exists.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to check
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, Piece, PieceType, Position};
    ///
    /// let mut board = Board::new();
    /// let pos = Position::new(0, 0);
    /// let piece = Piece { color: Color::White, piece_type: PieceType::Rook };
    ///
    /// board.set_piece(pos.clone(), piece.clone());
    /// assert_eq!(board.get_piece(&pos), Some(&piece));
    /// ```
    pub fn get_piece(&self, position: &Position) -> Option<&Piece> {
        self.pieces.get(position)
    }
    
    /// Places a piece at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position where the piece should be placed
    /// * `piece` - The piece to place
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, Piece, PieceType, Position};
    ///
    /// let mut board = Board::new();
    /// let pos = Position::new(0, 0);
    /// let piece = Piece { color: Color::White, piece_type: PieceType::Rook };
    ///
    /// board.set_piece(pos.clone(), piece.clone());
    /// assert_eq!(board.get_piece(&pos), Some(&piece));
    /// ```
    pub fn set_piece(&mut self, position: Position, piece: Piece) {
        self.pieces.insert(position, piece);
    }
    
    /// Removes a piece from the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position from which to remove the piece
    ///
    /// # Returns
    ///
    /// The removed piece, if one was present
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, Piece, PieceType, Position};
    ///
    /// let mut board = Board::new();
    /// let pos = Position::new(0, 0);
    /// let piece = Piece { color: Color::White, piece_type: PieceType::Rook };
    ///
    /// board.set_piece(pos.clone(), piece.clone());
    /// let removed = board.remove_piece(&pos);
    /// assert_eq!(removed, Some(piece));
    /// assert_eq!(board.get_piece(&pos), None);
    /// ```
    pub fn remove_piece(&mut self, position: &Position) -> Option<Piece> {
        self.pieces.remove(position)
    }
    
    /// Checks if the board is empty.
    ///
    /// # Returns
    ///
    /// `true` if there are no pieces on the board, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        assert!(board.pieces.is_empty());
    }

    #[test]
    fn test_new_game_has_32_pieces() {
        let board = Board::new_game();
        assert_eq!(board.pieces.len(), 32);
    }

    #[test]
    fn test_get_and_set_piece() {
        let mut board = Board::new();
        let pos = Position::new(3, 3);
        
        // Initially no piece at position
        assert_eq!(board.get_piece(&pos), None);
        
        // Set a piece
        let piece = Piece {
            color: Color::White,
            piece_type: PieceType::Queen,
        };
        board.set_piece(pos.clone(), piece.clone());
        
        // Verify piece is at position
        assert_eq!(board.get_piece(&pos), Some(&piece));
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::new();
        let pos = Position::new(3, 3);
        let piece = Piece {
            color: Color::White,
            piece_type: PieceType::Queen,
        };
        
        // Set a piece
        board.set_piece(pos.clone(), piece.clone());
        
        // Remove the piece
        let removed = board.remove_piece(&pos);
        assert_eq!(removed, Some(piece));
        
        // Verify piece is no longer at position
        assert_eq!(board.get_piece(&pos), None);
    }

    #[test]
    fn test_new_game_has_correct_pieces() {
        let board = Board::new_game();
        
        // Check white pawns
        for file in 0..8 {
            let pos = Position::new(file, 1);
            let piece = board.get_piece(&pos).unwrap();
            assert_eq!(piece.color, Color::White);
            assert_eq!(piece.piece_type, PieceType::Pawn);
        }
        
        // Check black pawns
        for file in 0..8 {
            let pos = Position::new(file, 6);
            let piece = board.get_piece(&pos).unwrap();
            assert_eq!(piece.color, Color::Black);
            assert_eq!(piece.piece_type, PieceType::Pawn);
        }
        
        // Check a few key pieces
        // White king
        let white_king_pos = Position::new(4, 0);
        let white_king = board.get_piece(&white_king_pos).unwrap();
        assert_eq!(white_king.color, Color::White);
        assert_eq!(white_king.piece_type, PieceType::King);
        
        // Black queen
        let black_queen_pos = Position::new(3, 7);
        let black_queen = board.get_piece(&black_queen_pos).unwrap();
        assert_eq!(black_queen.color, Color::Black);
        assert_eq!(black_queen.piece_type, PieceType::Queen);
    }
}

use std::collections::HashMap;
use crate::types::{Color, Piece, PieceType, Position};

/// Represents a chess board with pieces.
///
/// The board stores pieces in a HashMap where the keys are positions
/// and the values are the pieces at those positions.
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pieces: HashMap<Position, Piece>,
}

impl Board {
    /// Creates a new empty chess board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    ///
    /// let board = Board::new();
    /// assert!(board.is_empty());
    /// ```
    pub fn new() -> Self {
        Board {
            pieces: HashMap::new(),
        }
    }

    /// Creates a new chess board with pieces in their starting positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, PieceType, Position};
    ///
    /// let board = Board::new_game();
    /// 
    /// // Check if white pawns are set up correctly
    /// for file in 0..8 {
    ///     let pos = Position { file, rank: 1 };
    ///     let piece = board.get_piece(&pos).unwrap();
    ///     assert_eq!(piece.piece_type, PieceType::Pawn);
    ///     assert_eq!(piece.color, Color::White);
    /// }
    /// ```
    pub fn new_game() -> Self {
        let mut board = Board::new();
        
        // Set up pawns
        for file in 0..8 {
            board.set_piece(Position { file, rank: 1 }, Piece { piece_type: PieceType::Pawn, color: Color::White });
            board.set_piece(Position { file, rank: 6 }, Piece { piece_type: PieceType::Pawn, color: Color::Black });
        }
        
        // Set up rooks
        board.set_piece(Position { file: 0, rank: 0 }, Piece { piece_type: PieceType::Rook, color: Color::White });
        board.set_piece(Position { file: 7, rank: 0 }, Piece { piece_type: PieceType::Rook, color: Color::White });
        board.set_piece(Position { file: 0, rank: 7 }, Piece { piece_type: PieceType::Rook, color: Color::Black });
        board.set_piece(Position { file: 7, rank: 7 }, Piece { piece_type: PieceType::Rook, color: Color::Black });
        
        // Set up knights
        board.set_piece(Position { file: 1, rank: 0 }, Piece { piece_type: PieceType::Knight, color: Color::White });
        board.set_piece(Position { file: 6, rank: 0 }, Piece { piece_type: PieceType::Knight, color: Color::White });
        board.set_piece(Position { file: 1, rank: 7 }, Piece { piece_type: PieceType::Knight, color: Color::Black });
        board.set_piece(Position { file: 6, rank: 7 }, Piece { piece_type: PieceType::Knight, color: Color::Black });
        
        // Set up bishops
        board.set_piece(Position { file: 2, rank: 0 }, Piece { piece_type: PieceType::Bishop, color: Color::White });
        board.set_piece(Position { file: 5, rank: 0 }, Piece { piece_type: PieceType::Bishop, color: Color::White });
        board.set_piece(Position { file: 2, rank: 7 }, Piece { piece_type: PieceType::Bishop, color: Color::Black });
        board.set_piece(Position { file: 5, rank: 7 }, Piece { piece_type: PieceType::Bishop, color: Color::Black });
        
        // Set up queens
        board.set_piece(Position { file: 3, rank: 0 }, Piece { piece_type: PieceType::Queen, color: Color::White });
        board.set_piece(Position { file: 3, rank: 7 }, Piece { piece_type: PieceType::Queen, color: Color::Black });
        
        // Set up kings
        board.set_piece(Position { file: 4, rank: 0 }, Piece { piece_type: PieceType::King, color: Color::White });
        board.set_piece(Position { file: 4, rank: 7 }, Piece { piece_type: PieceType::King, color: Color::Black });
        
        board
    }
    
    /// Retrieves a piece at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to get the piece from
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the piece if one exists at the position,
    /// or `None` if no piece exists at the position.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, Piece, PieceType, Position};
    ///
    /// let mut board = Board::new();
    /// let pos = Position { file: 0, rank: 0 };
    /// let piece = Piece { piece_type: PieceType::Rook, color: Color::White };
    /// 
    /// board.set_piece(pos.clone(), piece.clone());
    /// assert_eq!(board.get_piece(&pos), Some(&piece));
    /// ```
    pub fn get_piece(&self, position: &Position) -> Option<&Piece> {
        self.pieces.get(position)
    }
    
    /// Places a piece at the specified position.
    ///
    /// If a piece already exists at the position, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to place the piece
    /// * `piece` - The piece to place
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, Piece, PieceType, Position};
    ///
    /// let mut board = Board::new();
    /// let pos = Position { file: 3, rank: 3 };
    /// let piece = Piece { piece_type: PieceType::Queen, color: Color::White };
    /// 
    /// board.set_piece(pos.clone(), piece.clone());
    /// assert_eq!(board.get_piece(&pos), Some(&piece));
    /// ```
    pub fn set_piece(&mut self, position: Position, piece: Piece) {
        self.pieces.insert(position, piece);
    }
    
    /// Removes a piece from the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to remove the piece from
    ///
    /// # Returns
    ///
    /// An `Option` containing the removed piece if one existed at the position,
    /// or `None` if no piece existed at the position.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, Piece, PieceType, Position};
    ///
    /// let mut board = Board::new();
    /// let pos = Position { file: 4, rank: 4 };
    /// let piece = Piece { piece_type: PieceType::Knight, color: Color::Black };
    /// 
    /// board.set_piece(pos.clone(), piece.clone());
    /// assert_eq!(board.remove_piece(&pos), Some(piece));
    /// assert_eq!(board.get_piece(&pos), None);
    /// ```
    pub fn remove_piece(&mut self, position: &Position) -> Option<Piece> {
        self.pieces.remove(position)
    }
    
    /// Checks if the board is empty (contains no pieces).
    ///
    /// # Returns
    ///
    /// `true` if the board has no pieces, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, Piece, PieceType, Position};
    ///
    /// let mut board = Board::new();
    /// assert!(board.is_empty());
    ///
    /// board.set_piece(
    ///     Position { file: 0, rank: 0 },
    ///     Piece { piece_type: PieceType::Pawn, color: Color::White }
    /// );
    /// assert!(!board.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        assert!(board.is_empty());
        assert_eq!(board.pieces.len(), 0);
    }
    
    #[test]
    fn test_new_game_has_correct_pieces() {
        let board = Board::new_game();
        assert_eq!(board.pieces.len(), 32); // 16 pieces per player
        
        // Check some specific pieces
        assert_eq!(
            board.get_piece(&Position { file: 0, rank: 0 }), 
            Some(&Piece { piece_type: PieceType::Rook, color: Color::White })
        );
        assert_eq!(
            board.get_piece(&Position { file: 4, rank: 0 }), 
            Some(&Piece { piece_type: PieceType::King, color: Color::White })
        );
        assert_eq!(
            board.get_piece(&Position { file: 3, rank: 7 }), 
            Some(&Piece { piece_type: PieceType::Queen, color: Color::Black })
        );
    }
    
    #[test]
    fn test_set_and_get_piece() {
        let mut board = Board::new();
        let pos = Position { file: 2, rank: 3 };
        let piece = Piece { piece_type: PieceType::Bishop, color: Color::White };
        
        // Initially no piece
        assert_eq!(board.get_piece(&pos), None);
        
        // Set piece and verify
        board.set_piece(pos.clone(), piece.clone());
        assert_eq!(board.get_piece(&pos), Some(&piece));
    }
    
    #[test]
    fn test_remove_piece() {
        let mut board = Board::new();
        let pos = Position { file: 5, rank: 5 };
        let piece = Piece { piece_type: PieceType::Knight, color: Color::Black };
        
        // Set and verify
        board.set_piece(pos.clone(), piece.clone());
        assert_eq!(board.get_piece(&pos), Some(&piece));
        
        // Remove and verify
        assert_eq!(board.remove_piece(&pos), Some(piece));
        assert_eq!(board.get_piece(&pos), None);
        
        // Remove again should return None
        assert_eq!(board.remove_piece(&pos), None);
    }
}

//! Chess board implementation with piece storage and basic manipulation methods.
//!
//! This module provides the core board representation and functionality for a chess game,
//! including piece placement, movement, and board state management.

use std::collections::HashMap;

use crate::types::{Color, Piece, PieceType, Position};

/// Represents a chess board with piece positions.
///
/// A board stores pieces in a HashMap for efficient lookup by position.
/// The standard chess board is an 8x8 grid with coordinates from (0,0) to (7,7).
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    /// Storage for pieces on the board, indexed by position
    pieces: HashMap<Position, Piece>,
}

impl Board {
    /// Creates a new empty chess board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    ///
    /// let board = Board::new();
    /// assert_eq!(board.count_pieces(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            pieces: HashMap::new(),
        }
    }

    /// Creates a new chess board with pieces in their standard starting positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::Color;
    ///
    /// let board = Board::new_game();
    /// assert_eq!(board.count_pieces(), 32);
    /// assert_eq!(board.count_pieces_by_color(Color::White), 16);
    /// assert_eq!(board.count_pieces_by_color(Color::Black), 16);
    /// ```
    pub fn new_game() -> Self {
        let mut board = Self::new();
        
        // Place pawns
        for file in 0..8 {
            board.set_piece(Position { rank: 1, file }, Piece { color: Color::White, piece_type: PieceType::Pawn });
            board.set_piece(Position { rank: 6, file }, Piece { color: Color::Black, piece_type: PieceType::Pawn });
        }
        
        // Place rooks
        board.set_piece(Position { rank: 0, file: 0 }, Piece { color: Color::White, piece_type: PieceType::Rook });
        board.set_piece(Position { rank: 0, file: 7 }, Piece { color: Color::White, piece_type: PieceType::Rook });
        board.set_piece(Position { rank: 7, file: 0 }, Piece { color: Color::Black, piece_type: PieceType::Rook });
        board.set_piece(Position { rank: 7, file: 7 }, Piece { color: Color::Black, piece_type: PieceType::Rook });
        
        // Place knights
        board.set_piece(Position { rank: 0, file: 1 }, Piece { color: Color::White, piece_type: PieceType::Knight });
        board.set_piece(Position { rank: 0, file: 6 }, Piece { color: Color::White, piece_type: PieceType::Knight });
        board.set_piece(Position { rank: 7, file: 1 }, Piece { color: Color::Black, piece_type: PieceType::Knight });
        board.set_piece(Position { rank: 7, file: 6 }, Piece { color: Color::Black, piece_type: PieceType::Knight });
        
        // Place bishops
        board.set_piece(Position { rank: 0, file: 2 }, Piece { color: Color::White, piece_type: PieceType::Bishop });
        board.set_piece(Position { rank: 0, file: 5 }, Piece { color: Color::White, piece_type: PieceType::Bishop });
        board.set_piece(Position { rank: 7, file: 2 }, Piece { color: Color::Black, piece_type: PieceType::Bishop });
        board.set_piece(Position { rank: 7, file: 5 }, Piece { color: Color::Black, piece_type: PieceType::Bishop });
        
        // Place queens
        board.set_piece(Position { rank: 0, file: 3 }, Piece { color: Color::White, piece_type: PieceType::Queen });
        board.set_piece(Position { rank: 7, file: 3 }, Piece { color: Color::Black, piece_type: PieceType::Queen });
        
        // Place kings
        board.set_piece(Position { rank: 0, file: 4 }, Piece { color: Color::White, piece_type: PieceType::King });
        board.set_piece(Position { rank: 7, file: 4 }, Piece { color: Color::Black, piece_type: PieceType::King });
        
        board
    }

    /// Returns a piece at the given position, if one exists.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to check for a piece
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Position, Color, PieceType};
    ///
    /// let board = Board::new_game();
    /// let pos = Position::from_notation("e1").unwrap();
    /// let piece = board.get_piece(&pos);
    /// assert!(piece.is_some());
    /// assert_eq!(piece.unwrap().piece_type, PieceType::King);
    /// assert_eq!(piece.unwrap().color, Color::White);
    /// ```
    pub fn get_piece(&self, position: &Position) -> Option<&Piece> {
        self.pieces.get(position)
    }

    /// Places a piece at the given position, replacing any existing piece.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to place the piece
    /// * `piece` - The piece to place
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Position, Color, Piece, PieceType};
    ///
    /// let mut board = Board::new();
    /// let pos = Position { rank: 3, file: 3 };
    /// let piece = Piece { color: Color::White, piece_type: PieceType::Queen };
    /// board.set_piece(pos, piece);
    /// assert_eq!(board.get_piece(&pos).unwrap().piece_type, PieceType::Queen);
    /// ```
    pub fn set_piece(&mut self, position: Position, piece: Piece) {
        self.pieces.insert(position, piece);
    }

    /// Removes a piece from the given position and returns it, if one exists.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to remove a piece from
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::Position;
    ///
    /// let mut board = Board::new_game();
    /// let pos = Position::from_notation("e2").unwrap();
    /// let piece = board.remove_piece(&pos);
    /// assert!(piece.is_some());
    /// assert!(board.get_piece(&pos).is_none());
    /// ```
    pub fn remove_piece(&mut self, position: &Position) -> Option<Piece> {
        self.pieces.remove(position)
    }

    /// Moves a piece from one position to another if the source position contains a piece.
    /// Returns true if a piece was moved, false otherwise.
    ///
    /// # Arguments
    ///
    /// * `from` - The source position
    /// * `to` - The target position
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::Position;
    ///
    /// let mut board = Board::new_game();
    /// let from = Position::from_notation("e2").unwrap();
    /// let to = Position::from_notation("e4").unwrap();
    /// assert!(board.move_piece(&from, &to));
    /// assert!(board.get_piece(&from).is_none());
    /// assert!(board.get_piece(&to).is_some());
    /// ```
    pub fn move_piece(&mut self, from: &Position, to: &Position) -> bool {
        if let Some(piece) = self.remove_piece(from) {
            self.set_piece(*to, piece);
            true
        } else {
            false
        }
    }

    /// Returns the total number of pieces on the board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    ///
    /// let board = Board::new_game();
    /// assert_eq!(board.count_pieces(), 32);
    /// ```
    pub fn count_pieces(&self) -> usize {
        self.pieces.len()
    }

    /// Returns the number of pieces of a specific color on the board.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of pieces to count
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::Color;
    ///
    /// let board = Board::new_game();
    /// assert_eq!(board.count_pieces_by_color(Color::White), 16);
    /// assert_eq!(board.count_pieces_by_color(Color::Black), 16);
    /// ```
    pub fn count_pieces_by_color(&self, color: Color) -> usize {
        self.pieces.values().filter(|p| p.color == color).count()
    }

    /// Checks if a position is within the bounds of the chess board (0-7, 0-7).
    ///
    /// # Arguments
    ///
    /// * `position` - The position to check
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::Position;
    ///
    /// let board = Board::new();
    /// assert!(board.is_valid_position(&Position { rank: 0, file: 0 }));
    /// assert!(board.is_valid_position(&Position { rank: 7, file: 7 }));
    /// assert!(!board.is_valid_position(&Position { rank: 8, file: 0 }));
    /// ```
    pub fn is_valid_position(&self, position: &Position) -> bool {
        position.rank < 8 && position.file < 8
    }

    /// Returns all pieces of a specific color on the board with their positions.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of pieces to find
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::Color;
    ///
    /// let board = Board::new_game();
    /// let white_pieces = board.get_pieces_by_color(Color::White);
    /// assert_eq!(white_pieces.len(), 16);
    /// ```
    pub fn get_pieces_by_color(&self, color: Color) -> Vec<(Position, &Piece)> {
        self.pieces
            .iter()
            .filter(|(_, piece)| piece.color == color)
            .map(|(pos, piece)| (*pos, piece))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        assert_eq!(board.count_pieces(), 0);
    }

    #[test]
    fn test_new_game_has_32_pieces() {
        let board = Board::new_game();
        assert_eq!(board.count_pieces(), 32);
        assert_eq!(board.count_pieces_by_color(Color::White), 16);
        assert_eq!(board.count_pieces_by_color(Color::Black), 16);
    }

    #[test]
    fn test_set_and_get_piece() {
        let mut board = Board::new();
        let pos = Position { rank: 3, file: 3 };
        let piece = Piece { color: Color::White, piece_type: PieceType::Queen };
        
        assert!(board.get_piece(&pos).is_none());
        board.set_piece(pos, piece);
        
        let retrieved_piece = board.get_piece(&pos);
        assert!(retrieved_piece.is_some());
        assert_eq!(retrieved_piece.unwrap().color, Color::White);
        assert_eq!(retrieved_piece.unwrap().piece_type, PieceType::Queen);
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::new();
        let pos = Position { rank: 3, file: 3 };
        let piece = Piece { color: Color::White, piece_type: PieceType::Queen };
        
        board.set_piece(pos, piece);
        assert!(board.get_piece(&pos).is_some());
        
        let removed_piece = board.remove_piece(&pos);
        assert!(removed_piece.is_some());
        assert_eq!(removed_piece.unwrap().piece_type, PieceType::Queen);
        assert!(board.get_piece(&pos).is_none());
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();
        let from = Position { rank: 1, file: 4 };
        let to = Position { rank: 3, file: 4 };
        let piece = Piece { color: Color::White, piece_type: PieceType::Pawn };
        
        board.set_piece(from, piece);
        assert!(board.move_piece(&from, &to));
        
        assert!(board.get_piece(&from).is_none());
        assert!(board.get_piece(&to).is_some());
        assert_eq!(board.get_piece(&to).unwrap().piece_type, PieceType::Pawn);
    }

    #[test]
    fn test_is_valid_position() {
        let board = Board::new();
        
        // Valid positions
        assert!(board.is_valid_position(&Position { rank: 0, file: 0 }));
        assert!(board.is_valid_position(&Position { rank: 7, file: 7 }));
        
        // Invalid positions
        assert!(!board.is_valid_position(&Position { rank: 8, file: 5 }));
        assert!(!board.is_valid_position(&Position { rank: 5, file: 8 }));
    }

    #[test]
    fn test_get_pieces_by_color() {
        let board = Board::new_game();
        
        let white_pieces = board.get_pieces_by_color(Color::White);
use std::collections::HashMap;

use crate::types::{Color, Piece, PieceType, Position};

/// A chess board that stores pieces in a HashMap.
///
/// The board uses a coordinate system where:
/// - x: 0-7 (a-h in chess notation)
/// - y: 0-7 (1-8 in chess notation)
///
/// # Examples
///
/// ```
/// use crate::board::Board;
/// use crate::types::{Position, Piece, PieceType, Color};
///
/// // Create an empty board
/// let mut board = Board::new();
///
/// // Add a white pawn to e2
/// let pos = Position::from_notation("e2").unwrap();
/// let pawn = Piece { color: Color::White, piece_type: PieceType::Pawn };
/// board.set_piece(pos, Some(pawn));
///
/// // Get the piece at e2
/// let piece = board.get_piece(&Position::from_notation("e2").unwrap());
/// assert!(piece.is_some());
/// ```

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    /// Storage of chess pieces indexed by their positions
    pieces: HashMap<Position, Piece>,
}

impl Board {
    /// Creates a new empty board with no pieces.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::board::Board;
    ///
    /// let board = Board::new();
    /// assert_eq!(board.get_piece_count(), 0);
    /// ```
    pub fn new() -> Self {
        Board {
            pieces: HashMap::new(),
        }
    }

    /// Creates a new board with pieces arranged in the standard initial chess position.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::board::Board;
    /// use crate::types::{Position, PieceType};
    ///
    /// let board = Board::new_game();
    /// 
    /// // Verify white rooks are in the correct positions
    /// let a1 = Position::from_notation("a1").unwrap();
    /// let h1 = Position::from_notation("h1").unwrap();
    /// 
    /// assert_eq!(board.get_piece(&a1).unwrap().piece_type, PieceType::Rook);
    /// assert_eq!(board.get_piece(&h1).unwrap().piece_type, PieceType::Rook);
    /// ```
    pub fn new_game() -> Self {
        let mut board = Board::new();
        
        // Set up white pieces
        board.set_piece(Position { x: 0, y: 0 }, Some(Piece { color: Color::White, piece_type: PieceType::Rook }));
        board.set_piece(Position { x: 1, y: 0 }, Some(Piece { color: Color::White, piece_type: PieceType::Knight }));
        board.set_piece(Position { x: 2, y: 0 }, Some(Piece { color: Color::White, piece_type: PieceType::Bishop }));
        board.set_piece(Position { x: 3, y: 0 }, Some(Piece { color: Color::White, piece_type: PieceType::Queen }));
        board.set_piece(Position { x: 4, y: 0 }, Some(Piece { color: Color::White, piece_type: PieceType::King }));
        board.set_piece(Position { x: 5, y: 0 }, Some(Piece { color: Color::White, piece_type: PieceType::Bishop }));
        board.set_piece(Position { x: 6, y: 0 }, Some(Piece { color: Color::White, piece_type: PieceType::Knight }));
        board.set_piece(Position { x: 7, y: 0 }, Some(Piece { color: Color::White, piece_type: PieceType::Rook }));

        // Set up white pawns
        for x in 0..8 {
            board.set_piece(Position { x, y: 1 }, Some(Piece { color: Color::White, piece_type: PieceType::Pawn }));
        }

        // Set up black pieces
        board.set_piece(Position { x: 0, y: 7 }, Some(Piece { color: Color::Black, piece_type: PieceType::Rook }));
        board.set_piece(Position { x: 1, y: 7 }, Some(Piece { color: Color::Black, piece_type: PieceType::Knight }));
        board.set_piece(Position { x: 2, y: 7 }, Some(Piece { color: Color::Black, piece_type: PieceType::Bishop }));
        board.set_piece(Position { x: 3, y: 7 }, Some(Piece { color: Color::Black, piece_type: PieceType::Queen }));
        board.set_piece(Position { x: 4, y: 7 }, Some(Piece { color: Color::Black, piece_type: PieceType::King }));
        board.set_piece(Position { x: 5, y: 7 }, Some(Piece { color: Color::Black, piece_type: PieceType::Bishop }));
        board.set_piece(Position { x: 6, y: 7 }, Some(Piece { color: Color::Black, piece_type: PieceType::Knight }));
        board.set_piece(Position { x: 7, y: 7 }, Some(Piece { color: Color::Black, piece_type: PieceType::Rook }));

        // Set up black pawns
        for x in 0..8 {
            board.set_piece(Position { x, y: 6 }, Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }));
        }

        board
    }

    /// Returns the piece at the specified position, or None if the position is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::board::Board;
    /// use crate::types::Position;
    ///
    /// let board = Board::new_game();
    /// let e2 = Position::from_notation("e2").unwrap();
    /// let piece = board.get_piece(&e2);
    /// assert!(piece.is_some());
    /// ```
    pub fn get_piece(&self, position: &Position) -> Option<Piece> {
        self.pieces.get(position).cloned()
    }

    /// Places a piece at the specified position. If `piece` is None, removes any piece at that position.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::board::Board;
    /// use crate::types::{Position, Piece, PieceType, Color};
    ///
    /// let mut board = Board::new();
    /// let pos = Position::from_notation("e4").unwrap();
    /// let knight = Piece { color: Color::White, piece_type: PieceType::Knight };
    ///
    /// // Place a knight at e4
    /// board.set_piece(pos, Some(knight));
    /// assert_eq!(board.get_piece(&pos).unwrap().piece_type, PieceType::Knight);
    ///
    /// // Remove the knight
    /// board.set_piece(pos, None);
    /// assert!(board.get_piece(&pos).is_none());
    /// ```
    pub fn set_piece(&mut self, position: Position, piece: Option<Piece>) {
        match piece {
            Some(p) => {
                self.pieces.insert(position, p);
            }
            None => {
                self.pieces.remove(&position);
            }
        }
    }

    /// Returns the total number of pieces on the board.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::board::Board;
    ///
    /// let board = Board::new_game();
    /// assert_eq!(board.get_piece_count(), 32); // Standard chess has 32 pieces at the start
    /// ```
    pub fn get_piece_count(&self) -> usize {
        self.pieces.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        assert_eq!(board.get_piece_count(), 0);
    }

    #[test]
    fn test_new_game_has_32_pieces() {
        let board = Board::new_game();
        assert_eq!(board.get_piece_count(), 32);
    }

    #[test]
    fn test_get_and_set_piece() {
        let mut board = Board::new();
        let pos = Position { x: 3, y: 3 }; // d4 in chess notation
        
        // Initially empty
        assert!(board.get_piece(&pos).is_none());
        
        // Set a piece
        let knight = Piece { color: Color::White, piece_type: PieceType::Knight };
        board.set_piece(pos, Some(knight));
        
        // Verify piece is there
        let retrieved = board.get_piece(&pos).unwrap();
        assert_eq!(retrieved.color, Color::White);
        assert_eq!(retrieved.piece_type, PieceType::Knight);
        
        // Remove piece
        board.set_piece(pos, None);
        assert!(board.get_piece(&pos).is_none());
    }

    #[test]
    fn test_new_game_initial_position() {
        let board = Board::new_game();
        
        // Check white pieces
        assert_eq!(
            board.get_piece(&Position { x: 0, y: 0 }).unwrap(),
            Piece { color: Color::White, piece_type: PieceType::Rook }
        );
        assert_eq!(
            board.get_piece(&Position { x: 4, y: 0 }).unwrap(),
            Piece { color: Color::White, piece_type: PieceType::King }
        );
        
        // Check white pawn
        assert_eq!(
            board.get_piece(&Position { x: 3, y: 1 }).unwrap(),
            Piece { color: Color::White, piece_type: PieceType::Pawn }
        );
        
        // Check black pieces
        assert_eq!(
            board.get_piece(&Position { x: 0, y: 7 }).unwrap(),
            Piece { color: Color::Black, piece_type: PieceType::Rook }
        );
        assert_eq!(
            board.get_piece(&Position { x: 4, y: 7 }).unwrap(),
            Piece { color: Color::Black, piece_type: PieceType::King }
        );
        
        // Check black pawn
        assert_eq!(
            board.get_piece(&Position { x: 3, y: 6 }).unwrap(),
            Piece { color: Color::Black, piece_type: PieceType::Pawn }
        );
        
        // Check empty square
        assert!(board.get_piece(&Position { x: 3, y: 3 }).is_none());
    }
}

//! Chess board representation and move validation logic
//!
//! This module contains the `Board` struct, which represents a chess board,
//! and all associated functions for manipulating the board and validating moves.

use std::collections::HashMap;
use std::fmt;

use crate::types::{Color, Piece, PieceType, Position};

/// Error types for invalid chess moves
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MoveError {
    /// No piece at the starting position
    NoPieceAtSource,
    /// Cannot move to a position occupied by a piece of the same color
    OccupiedByFriendly,
    /// The move is not valid for the piece's movement rules
    InvalidPieceMovement,
    /// The path to the destination is blocked by other pieces
    PathBlocked,
    /// The player is in check and the move doesn't resolve it
    MoveDoesntResolveCheck,
    /// The move would place the player's king in check
    MoveIntoCheck,
    /// Invalid castling attempt (king or rook moved, path blocked, etc.)
    InvalidCastling,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::NoPieceAtSource => write!(f, "No piece at source position"),
            MoveError::OccupiedByFriendly => write!(f, "Destination occupied by friendly piece"),
            MoveError::InvalidPieceMovement => write!(f, "Invalid movement for this piece type"),
            MoveError::PathBlocked => write!(f, "Path to destination is blocked"),
            MoveError::MoveDoesntResolveCheck => write!(f, "Move doesn't resolve check"),
            MoveError::MoveIntoCheck => write!(f, "Move would place king in check"),
            MoveError::InvalidCastling => write!(f, "Invalid castling attempt"),
        }
    }
}

/// Represents a movement from one position to another
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    /// Starting position of the move
    pub from: Position,
    /// Ending position of the move
    pub to: Position,
    /// Piece type to promote to if this is a pawn promotion
    pub promotion: Option<PieceType>,
}

impl Move {
    /// Create a new move from one position to another
    pub fn new(from: Position, to: Position) -> Self {
        Self {
            from,
            to,
            promotion: None,
        }
    }

    /// Create a new move with promotion information
    pub fn with_promotion(from: Position, to: Position, promotion: PieceType) -> Self {
        Self {
            from,
            to,
            promotion: Some(promotion),
        }
    }
}

/// Represents the state of a chess board, including pieces and movement history
#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    /// Map of positions to pieces currently on the board
    pieces: HashMap<Position, Piece>,
    /// Tracking if kings have moved (for castling)
    white_king_moved: bool,
    black_king_moved: bool,
    /// Tracking if rooks have moved (for castling)
    rooks_moved: HashMap<Position, bool>,
    /// Last move made on the board (for en passant detection)
    last_move: Option<Move>,
    /// Player whose turn it is
    current_turn: Color,
}

impl Board {
    /// Creates a new empty chess board
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// let board = Board::new();
    /// assert_eq!(board.piece_count(), 0);
    /// ```
}
    pub fn new() -> Self {
        Self {
            pieces: HashMap::new(),
            white_king_moved: false,
            black_king_moved: false,
            rooks_moved: HashMap::new(),
            last_move: None,
            current_turn: Color::White,
        }
    }

    /// Creates a new chess board with pieces in standard starting positions
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, PieceType, Position};
    ///
    /// let board = Board::new_game();
    /// // Check that white rook is at a1
    /// let a1 = Position::new(0, 0);
    /// let piece = board.get_piece(&a1).unwrap();
    /// assert_eq!(piece.piece_type, PieceType::Rook);
    /// assert_eq!(piece.color, Color::White);
    /// ```
    pub fn new_game() -> Self {
        let mut board = Self::new();
        
        // Place pawns
        for file in 0..8 {
            board.set_piece(Position::new(file, 1), Piece::new(PieceType::Pawn, Color::White));
            board.set_piece(Position::new(file, 6), Piece::new(PieceType::Pawn, Color::Black));
        }
        
        // Place rooks
        board.set_piece(Position::new(0, 0), Piece::new(PieceType::Rook, Color::White));
        board.set_piece(Position::new(7, 0), Piece::new(PieceType::Rook, Color::White));
        board.set_piece(Position::new(0, 7), Piece::new(PieceType::Rook, Color::Black));
        board.set_piece(Position::new(7, 7), Piece::new(PieceType::Rook, Color::Black));
        
        // Place knights
        board.set_piece(Position::new(1, 0), Piece::new(PieceType::Knight, Color::White));
        board.set_piece(Position::new(6, 0), Piece::new(PieceType::Knight, Color::White));
        board.set_piece(Position::new(1, 7), Piece::new(PieceType::Knight, Color::Black));
        board.set_piece(Position::new(6, 7), Piece::new(PieceType::Knight, Color::Black));
        
        // Place bishops
        board.set_piece(Position::new(2, 0), Piece::new(PieceType::Bishop, Color::White));
        board.set_piece(Position::new(5, 0), Piece::new(PieceType::Bishop, Color::White));
        board.set_piece(Position::new(2, 7), Piece::new(PieceType::Bishop, Color::Black));
        board.set_piece(Position::new(5, 7), Piece::new(PieceType::Bishop, Color::Black));
        
        // Place queens
        board.set_piece(Position::new(3, 0), Piece::new(PieceType::Queen, Color::White));
        board.set_piece(Position::new(3, 7), Piece::new(PieceType::Queen, Color::Black));
        
        // Place kings
        board.set_piece(Position::new(4, 0), Piece::new(PieceType::King, Color::White));
        board.set_piece(Position::new(4, 7), Piece::new(PieceType::King, Color::Black));
        
        board
    }

    /// Gets the piece at the specified position, if any
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::Position;
    ///
    /// let board = Board::new_game();
    /// let e4 = Position::from_notation("e4").unwrap();
    /// assert!(board.get_piece(&e4).is_none()); // e4 is empty at start
    /// ```
    pub fn get_piece(&self, pos: &Position) -> Option<Piece> {
        self.pieces.get(pos).cloned()
    }

    /// Sets a piece at the specified position
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::{Color, Piece, PieceType, Position};
    ///
    /// let mut board = Board::new();
    /// let e4 = Position::from_notation("e4").unwrap();
    /// board.set_piece(e4, Piece::new(PieceType::Pawn, Color::White));
    /// assert!(board.get_piece(&e4).is_some());
    /// ```
    pub fn set_piece(&mut self, pos: Position, piece: Piece) {
        self.pieces.insert(pos, piece);
    }

    /// Removes a piece at the specified position
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::types::Position;
    ///
    /// let mut board = Board::new_game();
    /// let e2 = Position::from_notation("e2").unwrap();
    /// assert!(board.get_piece(&e2).is_some());
    /// board.remove_piece(&e2);
    /// assert!(board.get_piece(&e2).is_none());
    /// ```
    pub fn remove_piece(&mut self, pos: &Position) -> Option<Piece> {
        self.pieces.remove(pos)
    }

    /// Returns the current player's color
    pub fn current_player(&self) -> Color {
        self.current_turn
    }

    /// Returns the number of pieces on the board
    pub fn piece_count(&self) -> usize {
        self.pieces.len()
    }

    /// Finds the position of the king of the given color
    fn find_king(&self, color: Color) -> Option<Position> {
        for (pos, piece) in &self.pieces {
            if piece.piece_type == PieceType::King && piece.color == color {
                return Some(*pos);
            }
        }
        None
    }

    /// Makes a move on the board if it's valid
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_app::board::Board;
    /// use chess_app::board::Move;
    /// use chess_app::types::Position;
    ///
    /// let mut board = Board::new_game();
    /// let e2 = Position::from_notation("e2").unwrap();
    /// let e4 = Position::from_notation("e4").unwrap();
    /// let chess_move = Move::new(e2, e4);
    /// let result = board.make_move(chess_move);
    /// assert!(result.is_ok());
    /// assert!(board.get_piece(&e4).is_some());
    /// assert!(board.get_piece(&e2).is_none());
    /// ```
    pub fn make_move(&mut self, chess_move: Move) -> Result<(), MoveError> {
        self.validate_move(&chess_move)?;
        
        let piece = self.get_piece(&chess_move.from).unwrap();
        
        // Handle special moves
        if piece.piece_type == PieceType::King {
            self.handle_king_move(&chess_move);
        } else if piece.piece_type == PieceType::Rook {
            self.handle_rook_move(&chess_move);
        } else if piece.piece_type == PieceType::Pawn {
            self.handle_pawn_move(&chess_move);
        }
        
        // Update piece position
        self.remove_piece(&chess_move.from);
        
        // Handle promotion
        if let Some(promotion_type) = chess_move.promotion {
            self.set_piece(chess_move.to, Piece::new(promotion_type, piece.color));
        } else {
            self.set_piece(chess_move.to, piece);
        }
        
        // Update last move for en passant tracking
        self.last_move = Some(chess_move);
        
        // Switch turn
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        
        Ok(())
    }

    /// Handle special king move cases (castling)
    fn handle_king_move(&mut self, chess_move: &Move) {
        let piece = self.get_piece(&chess_move.from).unwrap();
        
        // Update king moved status
        if piece.color == Color::White {
            self.white_king_moved = true;
        } else {
            self.black_king_moved = true;
        }
        
        // Check for castling (horizontal move of 2 squares)
        if (chess_move.to.file as i32 - chess_move.from.file as i32).abs() == 2 {
            let rank = chess_move.from.rank;
            
            // Kingside castling
            if chess_move.to.file > chess_move.from.file {
                // Move rook from h1/h8 to f1/f8
                let rook_from = Position::new(7, rank);
                let rook_to = Position::new(5, rank);
                let rook = self.remove_piece(&rook_from).unwrap();
                self.set_piece(rook_to, rook);
            } 
            // Queenside castling
            else {
                // Move rook from a1/a8 to d1/d8
                let rook_from = Position::new(0, rank);
                let rook_to = Position::new(3, rank);
                let rook = self.remove_piece(&rook_from).unwrap();
                self.set_piece(rook_to, rook);
            }
        }
    }

    /// Handle special rook move cases (tracking for castling)
    fn handle_rook_move(&mut self, chess_move: &Move) {
        // Mark rook as moved (for castling validation)
        self.rooks_moved.insert(chess_move.from, true);
    }

    /// Handle special pawn move cases (en passant, promotion)
    fn handle_pawn_move(&mut self, chess_move: &Move) {
        let piece = self.get_piece(&chess_move.from).unwrap();
        
        // Check for en passant capture
        if chess_move.from.file != chess_move.to.file && self.get_piece(&chess_move.to).is_none() {
            // This is a diagonal move to an empty square, must be en passant
            let capture_pos = Position::new(chess_move.to.file, chess_move.from.rank);
            self.remove_piece(&capture_pos);
        }
    }

    /// Validates if a move is legal
    pub fn validate_move(&self, chess_move: &Move) -> Result<(), MoveError> {
        // Get piece at start position
        let piece = self.get_piece(&chess_move.from)
            .ok_or(MoveError::NoPieceAtSource)?;
        
        // Check if it's the current player's turn
        if piece.color != self.current_turn {
            return Err(

//! Chess board implementation
//! 
//! This module implements a chess board and all associated functionality including:
//! - Basic board operations (initialization, piece manipulation)
//! - Move validation for all piece types
//! - Special move rules (castling, en passant, promotion)
//! - Check and checkmate detection

use crate::types::{Color, Piece, PieceType, Position};
use std::collections::HashMap;

/// Represents a chess board.
/// 
/// The board is represented as a HashMap where the keys are positions and
/// the values are pieces. This allows for efficient move validation and board manipulation.
#[derive(Clone, Debug)]
pub struct Board {
    /// Map of positions to pieces
    pieces: HashMap<Position, Piece>,
    /// Tracks if the white king has moved (for castling)
    white_king_moved: bool,
    /// Tracks if the black king has moved (for castling)
    black_king_moved: bool,
    /// Tracks if the white king's rook has moved (for castling)
    white_king_rook_moved: bool,
    /// Tracks if the white queen's rook has moved (for castling)
    white_queen_rook_moved: bool,
    /// Tracks if the black king's rook has moved (for castling)
    black_king_rook_moved: bool,
    /// Tracks if the black queen's rook has moved (for castling)
    black_queen_rook_moved: bool,
    /// Position for potential en passant capture
    en_passant_target: Option<Position>,
}

impl Board {
    /// Creates a new empty chess board.
    pub fn new() -> Self {
        Board {
            pieces: HashMap::new(),
            white_king_moved: false,
            black_king_moved: false,
            white_king_rook_moved: false,
            white_queen_rook_moved: false,
            black_king_rook_moved: false,
            black_queen_rook_moved: false,
            en_passant_target: None,
        }
    }

    /// Creates a new chess board with pieces in their starting positions.
    pub fn new_game() -> Self {
        let mut board = Board::new();
        
        // Place pawns
        for file in 0..8 {
            board.set_piece(Position { rank: 1, file }, Piece { color: Color::White, piece_type: PieceType::Pawn });
            board.set_piece(Position { rank: 6, file }, Piece { color: Color::Black, piece_type: PieceType::Pawn });
        }
        
        // Place rooks
        board.set_piece(Position { rank: 0, file: 0 }, Piece { color: Color::White, piece_type: PieceType::Rook });
        board.set_piece(Position { rank: 0, file: 7 }, Piece { color: Color::White, piece_type: PieceType::Rook });
        board.set_piece(Position { rank: 7, file: 0 }, Piece { color: Color::Black, piece_type: PieceType::Rook });
        board.set_piece(Position { rank: 7, file: 7 }, Piece { color: Color::Black, piece_type: PieceType::Rook });
        
        // Place knights
        board.set_piece(Position { rank: 0, file: 1 }, Piece { color: Color::White, piece_type: PieceType::Knight });
        board.set_piece(Position { rank: 0, file: 6 }, Piece { color: Color::White, piece_type: PieceType::Knight });
        board.set_piece(Position { rank: 7, file: 1 }, Piece { color: Color::Black, piece_type: PieceType::Knight });
        board.set_piece(Position { rank: 7, file: 6 }, Piece { color: Color::Black, piece_type: PieceType::Knight });
        
        // Place bishops
        board.set_piece(Position { rank: 0, file: 2 }, Piece { color: Color::White, piece_type: PieceType::Bishop });
        board.set_piece(Position { rank: 0, file: 5 }, Piece { color: Color::White, piece_type: PieceType::Bishop });
        board.set_piece(Position { rank: 7, file: 2 }, Piece { color: Color::Black, piece_type: PieceType::Bishop });
        board.set_piece(Position { rank: 7, file: 5 }, Piece { color: Color::Black, piece_type: PieceType::Bishop });
        
        // Place queens
        board.set_piece(Position { rank: 0, file: 3 }, Piece { color: Color::White, piece_type: PieceType::Queen });
        board.set_piece(Position { rank: 7, file: 3 }, Piece { color: Color::Black, piece_type: PieceType::Queen });
        
        // Place kings
        board.set_piece(Position { rank: 0, file: 4 }, Piece { color: Color::White, piece_type: PieceType::King });
        board.set_piece(Position { rank: 7, file: 4 }, Piece { color: Color::Black, piece_type: PieceType::King });
        
        board
    }

    /// Returns the piece at the given position, if any.
    pub fn get_piece(&self, pos: Position) -> Option<&Piece> {
        self.pieces.get(&pos)
    }

    /// Sets a piece at the given position.
    pub fn set_piece(&mut self, pos: Position, piece: Piece) {
        self.pieces.insert(pos, piece);
    }

    /// Removes a piece from the given position and returns it, if any.
    pub fn remove_piece(&mut self, pos: Position) -> Option<Piece> {
        self.pieces.remove(&pos)
    }

    /// Gets the position of the king for the given color.
    pub fn get_king_position(&self, color: Color) -> Option<Position> {
        for (pos, piece) in &self.pieces {
            if piece.piece_type == PieceType::King && piece.color == color {
                return Some(*pos);
            }
        }
        None
    }

    /// Makes a move on the board from one position to another.
    /// 
    /// This function does not validate the move - use is_valid_move() first to ensure 
    /// the move is legal.
    /// 
    /// Returns true if the move was successfully made.
    pub fn make_move(&mut self, from: Position, to: Position) -> bool {
        if let Some(piece) = self.remove_piece(from) {
            // Special case for castling
            if piece.piece_type == PieceType::King {
                match piece.color {
                    Color::White => self.white_king_moved = true,
                    Color::Black => self.black_king_moved = true,
                }
                
                // Handle castling move
                if from.file == 4 && (from.rank == 0 || from.rank == 7) {
                    // King-side castling
                    if to.file == 6 {
                        let rook_pos = Position { rank: from.rank, file: 7 };
                        if let Some(rook) = self.remove_piece(rook_pos) {
                            self.set_piece(Position { rank: from.rank, file: 5 }, rook);
                        }
                    }
                    // Queen-side castling
                    else if to.file == 2 {
                        let rook_pos = Position { rank: from.rank, file: 0 };
                        if let Some(rook) = self.remove_piece(rook_pos) {
                            self.set_piece(Position { rank: from.rank, file: 3 }, rook);
                        }
                    }
                }
            }
            
            // Track rook moves for castling
            if piece.piece_type == PieceType::Rook {
                if from == Position { rank: 0, file: 0 } {
                    self.white_queen_rook_moved = true;
                } else if from == Position { rank: 0, file: 7 } {
                    self.white_king_rook_moved = true;
                } else if from == Position { rank: 7, file: 0 } {
                    self.black_queen_rook_moved = true;
                } else if from == Position { rank: 7, file: 7 } {
                    self.black_king_rook_moved = true;
                }
            }
            
            // Handle en passant capture
            if piece.piece_type == PieceType::Pawn {
                // Check if this is a pawn moving two squares
                if (from.rank == 1 && to.rank == 3) || (from.rank == 6 && to.rank == 4) {
                    self.en_passant_target = Some(Position { 
                        rank: (from.rank + to.rank) / 2, 
                        file: from.file 
                    });
                } else {
                    self.en_passant_target = None;
                }
                
                // Check if this is an en passant capture
                if let Some(target) = self.en_passant_target {
                    if piece.piece_type == PieceType::Pawn && to == target {
                        // Remove the captured pawn
                        let capture_pos = Position {
                            rank: from.rank,
                            file: to.file,
                        };
                        self.remove_piece(capture_pos);
                    }
                }
            } else {
                self.en_passant_target = None;
            }
            
            // Actually move the piece to the destination
            self.set_piece(to, piece);
            
            true
        } else {
            false
        }
    }

    /// Checks if a move is valid according to chess rules.
    /// 
    /// Takes into account:
    /// - The piece's movement pattern
    /// - Obstructions on the path
    /// - Capture rules
    /// - Special moves like castling and en passant
    /// - Whether the move would leave the king in check
    pub fn is_valid_move(&self, from: Position, to: Position, current_player: Color) -> bool {
        // Check if the from position has a piece of the current player's color
        let piece = match self.get_piece(from) {
            Some(p) if p.color == current_player => p,
            _ => return false,
        };
        
        // Check if the to position is within the board
        if !to.is_valid() {
            return false;
        }
        
        // Cannot capture own piece
        if let Some(target_piece) = self.get_piece(to) {
            if target_piece.color == current_player {
                return false;
            }
        }
        
        // Validate based on piece type
        let valid_move = match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(from, to, piece.color),
            PieceType::Rook => self.is_valid_rook_move(from, to),
            PieceType::Knight => self.is_valid_knight_move(from, to),
            PieceType::Bishop => self.is_valid_bishop_move(from, to),
            PieceType::Queen => self.is_valid_queen_move(from, to),
            PieceType::King => self.is_valid_king_move(from, to, piece.color),
        };
        
        if !valid_move {
            return false;
        }
        
        // Make a temporary move and check if it leaves the king in check
        let mut temp_board = self.clone();
        temp_board.make_move(from, to);
        
        !temp_board.is_in_check(current_player)
    }

    /// Checks if a pawn move is valid.
    fn is_valid_pawn_move(&self, from: Position, to: Position, color: Color) -> bool {
        let direction = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        
        let file_diff = to.file as i8 - from.file as i8;
        let rank_diff = to.rank as i8 - from.rank as i8;
        
        // Normal forward move
        if file_diff == 0 && rank_diff == direction {
            return self.get_piece(to).is_none();
        }
        
        // Initial two-square move
        if file_diff == 0 && 
           ((color == Color::White && from.rank == 1 && to.rank == 3) ||
            (color == Color::Black && from.rank == 6 && to.rank == 4)) {
            let intermediate = Position {
                rank: (from.rank as i8 + direction) as u8,
                file: from.file,
            };
            return self.get_piece(intermediate).is_none() && self.get_piece(to).is_none();
        }
        
        // Capturing diagonally
        if file_diff.abs() == 1 && rank_diff == direction {
            // Regular capture
            if self.get_piece(to).is_some() {
                return true;
            }
            
            // En passant capture
            if let Some(en_passant) = self.en_passant_target {
                if to == en_passant {
                    return true;
                }
            }
        }
        
        false
    }

    /// Checks if a rook move is valid.
    fn is_valid_rook_move(&self, from: Position, to: Position) -> bool {
        // Rook moves horizontally or vertically
        if from.rank != to.rank && from.file != to.file {
            return false;
        }
        
        // Check for pieces in the path
        if from.rank == to.rank {
            // Horizontal move
            let min_file = from.file.min(to.file);
            let max_file = from.file.max(to.file);
            for file in min_file+1..max_file {
                if self.get_piece(Position { rank: from.rank, file }).is_some() {
                    return false;
                }
            }
        } else {
            // Vertical move
            let min_rank = from.rank.min(to.rank);
            let max_rank = from.rank.max(to.rank);
            for rank in min_rank+1..max_rank {
                if self.get_piece(Position { rank, file: from.file }).is_some() {
                    return false;
                }
            }
        }
        
        true
    }

    /// Checks if a knight move is valid.
    fn is_valid_knight_move(&self, from: Position, to: Position) -> bool {
        let file_diff = (from.file as i8 - to.file as i8).abs();
        let rank_diff = (from.rank as i8 - to.rank as i8).abs();
        
        // Knights move in an L-shape: 2 squares in one direction and 1 square perpendicular
        (file_diff == 1 && rank_diff == 2) || (file_diff == 2 && rank

//! Chess board implementation
//!
//! This module contains the implementation of the chess board and all related
//! move validation and game state checking logic.

use crate::types::{Color, Piece, PieceType, Position};
use crate::state::GameState;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a chess board with piece positions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
}

impl Board {
    /// Creates a new empty chess board
    pub fn new() -> Self {
        Board {
            pieces: [[None; 8]; 8],
        }
    }

    /// Creates a new chess board with pieces in their standard starting positions
    pub fn new_game() -> Self {
        let mut board = Board::new();
        
        // Set up white pieces
        // Back rank
        board.set_piece(Position { rank: 0, file: 0 }, Some(Piece { piece_type: PieceType::Rook, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 1 }, Some(Piece { piece_type: PieceType::Knight, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 2 }, Some(Piece { piece_type: PieceType::Bishop, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 3 }, Some(Piece { piece_type: PieceType::Queen, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 4 }, Some(Piece { piece_type: PieceType::King, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 5 }, Some(Piece { piece_type: PieceType::Bishop, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 6 }, Some(Piece { piece_type: PieceType::Knight, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 7 }, Some(Piece { piece_type: PieceType::Rook, color: Color::White }));
        
        // White pawns
        for file in 0..8 {
            board.set_piece(
                Position { rank: 1, file },
                Some(Piece { piece_type: PieceType::Pawn, color: Color::White })
            );
        }
        
        // Set up black pieces
        // Back rank
        board.set_piece(Position { rank: 7, file: 0 }, Some(Piece { piece_type: PieceType::Rook, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 1 }, Some(Piece { piece_type: PieceType::Knight, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 2 }, Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 3 }, Some(Piece { piece_type: PieceType::Queen, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 4 }, Some(Piece { piece_type: PieceType::King, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 5 }, Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 6 }, Some(Piece { piece_type: PieceType::Knight, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 7 }, Some(Piece { piece_type: PieceType::Rook, color: Color::Black }));
        
        // Black pawns
        for file in 0..8 {
            board.set_piece(
                Position { rank: 6, file },
                Some(Piece { piece_type: PieceType::Pawn, color: Color::Black })
            );
        }
        
        board
    }

    /// Gets the piece at a given position, returns None if position is invalid or empty
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.is_valid() {
            self.pieces[pos.rank as usize][pos.file as usize]
        } else {
            None
        }
    }

    /// Sets a piece at a given position
    /// Returns true if successful, false if position is invalid
    pub fn set_piece(&mut self, pos: Position, piece: Option<Piece>) -> bool {
        if pos.is_valid() {
            self.pieces[pos.rank as usize][pos.file as usize] = piece;
            true
        } else {
            false
        }
    }

    /// Validates if a move from one position to another is legal according to chess rules
    pub fn is_valid_move(&self, from: Position, to: Position, current_player: Color, game_state: &GameState) -> bool {
        // Check if positions are within bounds
        if !from.is_valid() || !to.is_valid() {
            return false;
        }
        
        // Get the piece at the starting position
        let piece = match self.get_piece(from) {
            Some(p) => p,
            None => return false, // No piece at starting position
        };
        
        // Check if it's the correct player's turn
        if piece.color != current_player {
            return false;
        }
        
        // Check if destination contains same-colored piece
        if let Some(dest_piece) = self.get_piece(to) {
            if self.is_same_color(piece.color, dest_piece.color) {
                return false;
            }
        }
        
        // Validate move based on piece type
        let basic_move_valid = match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(from, to, piece.color, game_state),
            PieceType::Knight => self.is_valid_knight_move(from, to),
            PieceType::Bishop => self.is_valid_bishop_move(from, to),
            PieceType::Rook => self.is_valid_rook_move(from, to),
            PieceType::Queen => self.is_valid_queen_move(from, to),
            PieceType::King => self.is_valid_king_move(from, to, piece.color, game_state),
        };
        
        if !basic_move_valid {
            return false;
        }
        
        // Create a temporary board to test if the move would leave the king in check
        let mut test_board = self.clone();
        test_board.set_piece(to, Some(piece));
        test_board.set_piece(from, None);
        
        // Make sure the move doesn't leave or put the king in check
        if test_board.is_in_check(current_player) {
            return false;
        }
        
        true
    }
    
    /// Checks if there are any pieces between two positions (not including the endpoints)
    fn is_path_clear(&self, from: Position, to: Position) -> bool {
        let dx = (to.file - from.file).signum();
        let dy = (to.rank - from.rank).signum();
        
        let mut current = Position {
            file: from.file + dx,
            rank: from.rank + dy,
        };
        
        while current != to {
            if self.get_piece(current).is_some() {
                return false;
            }
            current.file += dx;
            current.rank += dy;
        }
        
        true
    }
    
    /// Checks if two colors are the same
    fn is_same_color(&self, color1: Color, color2: Color) -> bool {
        color1 == color2
    }
    
    /// Gets the rank difference between two positions
    fn rank_distance(&self, from: Position, to: Position) -> i8 {
        (to.rank - from.rank).abs()
    }
    
    /// Gets the file difference between two positions
    fn file_distance(&self, from: Position, to: Position) -> i8 {
        (to.file - from.file).abs()
    }
    
    /// Checks if a move is diagonal
    fn is_diagonal_move(&self, from: Position, to: Position) -> bool {
        self.rank_distance(from, to) == self.file_distance(from, to)
    }
    
    /// Checks if a move is straight (horizontal or vertical)
    fn is_straight_move(&self, from: Position, to: Position) -> bool {
        from.rank == to.rank || from.file == to.file
    }
    
    // Piece-specific validation methods
    
    /// Validates if a pawn move is legal
    fn is_valid_pawn_move(&self, from: Position, to: Position, color: Color, game_state: &GameState) -> bool {
        let forward_direction = match color {
            Color::White => 1,  // White pawns move up the board
            Color::Black => -1, // Black pawns move down the board
        };
        
        let rank_diff = to.rank - from.rank;
        let file_diff = self.file_distance(from, to);
        
        // Basic forward movement
        if file_diff == 0 {
            // Single step forward
            if rank_diff == forward_direction {
                if self.get_piece(to).is_some() {
                    return false; // Can't move forward if blocked
                }
                
                // Check for promotion (pawn reaching the last rank)
                let last_rank = match color {
                    Color::White => 7,
                    Color::Black => 0,
                };
                
                if to.rank == last_rank {
                    // Promotion is handled during move execution, here we just validate it
                    return true;
                }
                
                return true;
            }
            
            // Double step from starting position
            let is_starting_position = match color {
                Color::White => from.rank == 1,
                Color::Black => from.rank == 6,
            };
            
            if is_starting_position && rank_diff == 2 * forward_direction {
                let intermediate = Position {
                    rank: from.rank + forward_direction,
                    file: from.file,
                };
                return self.get_piece(intermediate).is_none() && self.get_piece(to).is_none();
            }
        }
        
        // Diagonal capture (including en passant)
        else if file_diff == 1 && rank_diff == forward_direction {
            // Regular capture
            if let Some(piece) = self.get_piece(to) {
                return !self.is_same_color(piece.color, color);
            }
            
            // En passant capture
            if let Some(last_move) = game_state.get_last_move() {
                // Check if the last move was a pawn moving two squares
                if let Some(last_piece) = self.get_piece(last_move.to) {
                    if last_piece.piece_type == PieceType::Pawn {
                        let expected_rank = match color {
                            Color::White => 4, // Black pawn must be on rank 5 (index 4)
                            Color::Black => 3, // White pawn must be on rank 4 (index 3)
                        };
                        
                        if last_move.to.rank == expected_rank 
                            && last_move.to.file == to.file
                            && (last_move.from.rank - last_move.to.rank).abs() == 2 {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
    
    /// Validates if a knight move is legal
    fn is_valid_knight_move(&self, from: Position, to: Position) -> bool {
        let rank_diff = self.rank_distance(from, to);
        let file_diff = self.file_distance(from, to);
        
        // Knight moves in an L-shape: 2 squares in one direction and 1 square perpendicular
        (rank_diff == 2 && file_diff == 1) || (rank_diff == 1 && file_diff == 2)
    }
    
    /// Validates if a bishop move is legal
    fn is_valid_bishop_move(&self, from: Position, to: Position) -> bool {
        // Bishop moves diagonally
        if !self.is_diagonal_move(from, to) {
            return false;
        }
        
        // Check if the path is clear
        self.is_path_clear(from, to)
    }
    
    /// Validates if a rook move is legal
    fn is_valid_rook_move(&self, from: Position, to: Position) -> bool {
        // Rook moves in straight lines
        if !self.is_straight_move(from, to) {
            return false;
        }
        
        // Check if the path is clear
        self.is_path_clear(from, to)
    }
    
    /// Validates if a queen move is legal
    fn is_valid_queen_move(&self, from: Position, to: Position) -> bool {
        // Queen can move like a bishop or rook
        if !self.is_diagonal_move(from, to) && !self.is_straight_move(from, to) {
            return false;
        }
        
        // Check if the path is clear
        self.is_path_clear(from, to)
    }
    
    /// Validates if a king move is legal, including castling
    fn is_valid_king_move(&self, from: Position, to: Position, color: Color, game_state: &GameState) -> bool {
        let rank_diff = self.rank_distance(from, to);
        let file_diff = self.file_distance(from, to);
        
        // Regular king move: one square in any direction
        if rank_diff <= 1 && file_diff <= 1 {
            return true;
        }
        
        // Castling
        if rank_diff == 0 && file_diff == 2 {
            // Check if king has moved
            if game_state.has_piece_moved(from) {
                return false;
            }
            
            let king_rank = match color {
                Color::White => 0,
                Color::Black => 7,
            };
            
            // Ensure king is in correct starting position
            if from.rank != king_rank || from.file != 4 {
                return false;
            }
            
            // Determine if kingside or queenside castling
            let (rook_file, path_range, through_squares) = if to.file == 6 {
                // Kingside castling
                (7, 5..7, vec![Position {

//! Chess board implementation
//!
//! This module contains the representation of a chess board and all the logic
//! for validating chess moves according to standard rules, including special
//! moves like castling, en passant captures, and pawn promotion.

use std::ops::Range;
use crate::types::{Color, Piece, PieceType, Position};
use crate::state::GameState;

/// Represents a chess board with pieces
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Board {
    /// The chess board represented as a 2D array of optional pieces
    /// First index is rank (0-7), second is file (0-7)
    pub squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    /// Creates a new empty chess board
    pub fn new() -> Self {
        Board {
            squares: [[None; 8]; 8],
        }
    }
    
    /// Creates a new chess board with pieces in their starting positions
    pub fn new_game() -> Self {
        let mut board = Board::new();
        
        // Set up white pieces
        // Back rank
        board.set_piece(Position { rank: 0, file: 0 }, Some(Piece { piece_type: PieceType::Rook, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 1 }, Some(Piece { piece_type: PieceType::Knight, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 2 }, Some(Piece { piece_type: PieceType::Bishop, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 3 }, Some(Piece { piece_type: PieceType::Queen, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 4 }, Some(Piece { piece_type: PieceType::King, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 5 }, Some(Piece { piece_type: PieceType::Bishop, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 6 }, Some(Piece { piece_type: PieceType::Knight, color: Color::White }));
        board.set_piece(Position { rank: 0, file: 7 }, Some(Piece { piece_type: PieceType::Rook, color: Color::White }));
        
        // White pawns
        for file in 0..8 {
            board.set_piece(
                Position { rank: 1, file },
                Some(Piece { piece_type: PieceType::Pawn, color: Color::White })
            );
        }
        
        // Set up black pieces
        // Back rank
        board.set_piece(Position { rank: 7, file: 0 }, Some(Piece { piece_type: PieceType::Rook, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 1 }, Some(Piece { piece_type: PieceType::Knight, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 2 }, Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 3 }, Some(Piece { piece_type: PieceType::Queen, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 4 }, Some(Piece { piece_type: PieceType::King, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 5 }, Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 6 }, Some(Piece { piece_type: PieceType::Knight, color: Color::Black }));
        board.set_piece(Position { rank: 7, file: 7 }, Some(Piece { piece_type: PieceType::Rook, color: Color::Black }));
        
        // Black pawns
        for file in 0..8 {
            board.set_piece(
                Position { rank: 6, file },
                Some(Piece { piece_type: PieceType::Pawn, color: Color::Black })
            );
        }
        
        board
    }

    /// Gets the piece at the specified position
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.is_valid() {
            self.squares[pos.rank as usize][pos.file as usize]
        } else {
            None
        }
    }

    /// Sets a piece at the specified position
    /// 
    /// Returns true if the position was valid and the piece was set
    pub fn set_piece(&mut self, pos: Position, piece: Option<Piece>) -> bool {
        if pos.is_valid() {
            self.squares[pos.rank as usize][pos.file as usize] = piece;
            true
        } else {
            false
        }
    }
    
    /// Validates if a move from one position to another is legal according to chess rules
    pub fn is_valid_move(&self, from: Position, to: Position, current_player: Color, game_state: &GameState) -> bool {
        // Check if positions are within bounds
        if !from.is_valid() || !to.is_valid() {
            return false;
        }
        
        // Get the piece at the starting position
        let piece = match self.get_piece(from) {
            Some(p) => p,
            None => return false, // No piece at starting position
        };
        
        // Check if it's the correct player's turn
        if piece.color != current_player {
            return false;
        }
        
        // Check if destination contains same-colored piece
        if let Some(dest_piece) = self.get_piece(to) {
            if self.is_same_color(piece.color, dest_piece.color) {
                return false;
            }
        }
        
        // Validate move based on piece type
        let basic_move_valid = match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(from, to, piece.color, game_state),
            PieceType::Knight => self.is_valid_knight_move(from, to),
            PieceType::Bishop => self.is_valid_bishop_move(from, to),
            PieceType::Rook => self.is_valid_rook_move(from, to),
            PieceType::Queen => self.is_valid_queen_move(from, to),
            PieceType::King => self.is_valid_king_move(from, to, piece.color, game_state),
        };
        
        if !basic_move_valid {
            return false;
        }
        
        // Create a temporary board to test if the move would leave the king in check
        let mut test_board = self.clone();
        test_board.set_piece(to, Some(piece));
        test_board.set_piece(from, None);
        
        // Make sure the move doesn't leave or put the king in check
        if test_board.is_in_check(current_player) {
            return false;
        }
        
        true
    }
    
    /// Checks if there are any pieces between two positions (not including the endpoints)
    fn is_path_clear(&self, from: Position, to: Position) -> bool {
        let dx = (to.file - from.file).signum();
        let dy = (to.rank - from.rank).signum();
        
        let mut current = Position {
            file: from.file + dx,
            rank: from.rank + dy,
        };
        
        while current != to {
            if self.get_piece(current).is_some() {
                return false;
            }
            current.file += dx;
            current.rank += dy;
        }
        
        true
    }
    
    /// Checks if two colors are the same
    fn is_same_color(&self, color1: Color, color2: Color) -> bool {
        color1 == color2
    }
    
    /// Gets the rank difference between two positions
    fn rank_distance(&self, from: Position, to: Position) -> i8 {
        (to.rank - from.rank).abs()
    }
    
    /// Gets the file difference between two positions
    fn file_distance(&self, from: Position, to: Position) -> i8 {
        (to.file - from.file).abs()
    }
    
    /// Checks if a move is diagonal
    fn is_diagonal_move(&self, from: Position, to: Position) -> bool {
        self.rank_distance(from, to) == self.file_distance(from, to)
    }
    
    /// Checks if a move is straight (horizontal or vertical)
    fn is_straight_move(&self, from: Position, to: Position) -> bool {
        from.rank == to.rank || from.file == to.file
    }
    
    // Piece-specific validation methods
    
    /// Validates a pawn move according to chess rules
    fn is_valid_pawn_move(&self, from: Position, to: Position, color: Color, game_state: &GameState) -> bool {
        let forward_direction = match color {
            Color::White => 1,  // White pawns move up the board
            Color::Black => -1, // Black pawns move down the board
        };
        
        let rank_diff = to.rank - from.rank;
        let file_diff = self.file_distance(from, to);
        
        // Basic forward movement
        if file_diff == 0 {
            // Single step forward
            if rank_diff == forward_direction {
                if self.get_piece(to).is_some() {
                    return false; // Can't move forward if blocked
                }
                
                // Check for promotion (pawn reaching the last rank)
                let last_rank = match color {
                    Color::White => 7,
                    Color::Black => 0,
                };
                
                if to.rank == last_rank {
                    // Promotion is handled during move execution, here we just validate it
                    return true;
                }
                
                return true;
            }
            
            // Double step from starting position
            let is_starting_position = match color {
                Color::White => from.rank == 1,
                Color::Black => from.rank == 6,
            };
            
            if is_starting_position && rank_diff == 2 * forward_direction {
                let intermediate = Position {
                    rank: from.rank + forward_direction,
                    file: from.file,
                };
                return self.get_piece(intermediate).is_none() && self.get_piece(to).is_none();
            }
        }
        
        // Diagonal capture (including en passant)
        else if file_diff == 1 && rank_diff == forward_direction {
            // Regular capture
            if let Some(piece) = self.get_piece(to) {
                return !self.is_same_color(piece.color, color);
            }
            
            // En passant capture
            if let Some(last_move) = game_state.get_last_move() {
                // Check if the last move was a pawn moving two squares
                if let Some(last_piece) = self.get_piece(last_move.to) {
                    if last_piece.piece_type == PieceType::Pawn {
                        let expected_rank = match color {
                            Color::White => 4, // Black pawn must be on rank 5 (index 4)
                            Color::Black => 3, // White pawn must be on rank 4 (index 3)
                        };
                        
                        if last_move.to.rank == expected_rank 
                            && last_move.to.file == to.file
                            && (last_move.from.rank - last_move.to.rank).abs() == 2 {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
    
    /// Validates a knight move according to chess rules
    fn is_valid_knight_move(&self, from: Position, to: Position) -> bool {
        let rank_diff = self.rank_distance(from, to);
        let file_diff = self.file_distance(from, to);
        
        // Knight moves in an L-shape: 2 squares in one direction and 1 square perpendicular
        (rank_diff == 2 && file_diff == 1) || (rank_diff == 1 && file_diff == 2)
    }
    
    /// Validates a bishop move according to chess rules
    fn is_valid_bishop_move(&self, from: Position, to: Position) -> bool {
        // Bishop moves diagonally
        if !self.is_diagonal_move(from, to) {
            return false;
        }
        
        // Check if the path is clear
        self.is_path_clear(from, to)
    }
    
    /// Validates a rook move according to chess rules
    fn is_valid_rook_move(&self, from: Position, to: Position) -> bool {
        // Rook moves in straight lines
        if !self.is_straight_move(from, to) {
            return false;
        }
        
        // Check if the path is clear
        self.is_path_clear(from, to)
    }
    
    /// Validates a queen move according to chess rules
    fn is_valid_queen_move(&self, from: Position, to: Position) -> bool {
        // Queen can move like a bishop or rook
        if !self.is_diagonal_move(from, to) && !self.is_straight_move(from, to) {
            return false;
        }
        
        // Check if the path is clear
        self.is_path_clear(from, to)
    }
    
    /// Validates a king move according to chess rules, including castling
    fn is_valid_king_move(&self, from: Position, to: Position, color: Color, game_state: &GameState) -> bool {
        let rank_diff = self.rank_distance(from, to);
        let file_diff = self.file_distance(from, to);
        
        // Regular king move: one square in any direction
        if rank_diff <= 1 && file_diff <= 1 {
            return true;
        }
        
        // Castling
        if rank_diff == 0 && file_diff == 2 {
            // Check if king has moved
            if game_state.has_piece_moved(from) {
                return false;
            }
            
            let king_rank = match color {
                Color::White => 0,
                Color::Black => 7,
            };
            
            // Ensure king is in correct starting position
            if from.rank != king_rank || from.file != 4 {
                return false;
            }
            
            // Determine if kingside or queenside castling
            let (rook_file, path_range, through_squares) = if to.file == 6 {
                // Kingside castling
                (7, 5..7, vec![Position

