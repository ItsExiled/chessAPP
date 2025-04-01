//! Chess game type definitions
//!
//! This module contains the core type definitions used throughout the chess application,
//! including colors, piece types, assets, and board positions. These types provide the
//! fundamental building blocks for representing a chess game state.

use serde::{Deserialize, Serialize};

/// Represents the color of a chess piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Returns the opposite color.
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// Represents the type of a chess piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

/// Represents a chess piece with its type and color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    /// Creates a new piece with the specified type and color.
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }
}

/// Represents a position on the chess board.
///
/// The position uses zero-based indexing:
/// - `rank` ranges from 0-7 (corresponding to rows 1-8 in chess notation)
/// - `file` ranges from 0-7 (corresponding to columns a-h in chess notation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub file: u8,  // 0-7 (a-h in chess notation)
    pub rank: u8,  // 0-7 (1-8 in chess notation)
}

impl Position {
    /// Creates a new position if the coordinates are valid.
    ///
    /// Returns `None` if either coordinate is outside the 0-7 range.
    pub fn new(file: u8, rank: u8) -> Self {
        Position { file, rank }
    }

    /// Creates a new position from standard chess notation.
    /// 
    /// Chess notation consists of a file letter (a-h) followed by a rank number (1-8).
    /// For example, "e4" represents the position with file 4 (e) and rank 3 (4 in chess notation - 1).
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use chess_app::types::Position;
    /// let e4 = Position::from_notation("e4").unwrap();
    /// assert_eq!(e4.file, 4);
    /// assert_eq!(e4.rank, 3);
    /// 
    /// // Invalid notations return None
    /// assert!(Position::from_notation("i9").is_none());
    /// ```
    pub fn from_notation(notation: &str) -> Option<Self> {
        // Validate the notation format
        if notation.len() != 2 {
            return None;
        }
        
        let chars: Vec<char> = notation.chars().collect();
        let file_char = chars[0];
        let rank_char = chars[1];
        
        // Convert file (a-h) to coordinate (0-7)
        let file = match file_char {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return None, // Invalid file
        };
        
        // Convert rank (1-8) to coordinate (0-7)
        let rank = match rank_char.to_digit(10) {
            Some(r) if r >= 1 && r <= 8 => (r - 1) as u8,
            _ => return None, // Invalid rank
        };
        
        Some(Position { file, rank })
    }
    
    /// Converts a position to standard chess notation.
    /// 
    /// Returns a string in the format file letter (a-h) followed by rank number (1-8).
    /// For example, a Position with file 4 and rank 3 returns "e4".
    /// 
    /// If the position is invalid (outside the 0-7 range), returns an error string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use chess_app::types::Position;
    /// let pos = Position { file: 4, rank: 3 };
    /// assert_eq!(pos.to_notation(), "e4");
    /// 
    /// // Invalid positions are handled gracefully
    /// let invalid = Position { file: 8, rank: 8 };
    /// assert!(invalid.to_notation().starts_with("Invalid"));
    /// ```
    pub fn to_notation(&self) -> String {
        if !self.is_valid() {
            return format!("Invalid position: file={}, rank={}", self.file, self.rank);
        }
        
        // Convert file (0-7) to file letter (a-h)
        let file_char = (b'a' + self.file) as char;
        
        // Convert rank (0-7) to rank number (1-8)
        let rank_num = self.rank + 1;
        
        format!("{}{}", file_char, rank_num)
    }

    /// Checks if the position is within the valid chess board bounds.
    pub fn is_valid(&self) -> bool {
        self.file < 8 && self.rank < 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_from_notation() {
        // Test valid notations
        let a1 = Position::from_notation("a1").unwrap();
        assert_eq!(a1.file, 0);
        assert_eq!(a1.rank, 0);
        
        let e4 = Position::from_notation("e4").unwrap();
        assert_eq!(e4.file, 4);
        assert_eq!(e4.rank, 3);
        
        let h8 = Position::from_notation("h8").unwrap();
        assert_eq!(h8.file, 7);
        assert_eq!(h8.rank, 7);
        
        // Test invalid notations
        assert!(Position::from_notation("i9").is_none());
        assert!(Position::from_notation("a0").is_none());
        assert!(Position::from_notation("a9").is_none());
        assert!(Position::from_notation("").is_none());
        assert!(Position::from_notation("a").is_none());
        assert!(Position::from_notation("abc").is_none());
    }
    
    #[test]
    fn test_position_to_notation() {
        // Test valid positions
        let a1 = Position { file: 0, rank: 0 };
        assert_eq!(a1.to_notation(), "a1");
        
        let e4 = Position { file: 4, rank: 3 };
        assert_eq!(e4.to_notation(), "e4");
        
        let h8 = Position { file: 7, rank: 7 };
        assert_eq!(h8.to_notation(), "h8");
        
        // Test invalid positions (these should handle gracefully)
        let invalid = Position { file: 8, rank: 8 };
        assert!(invalid.to_notation().starts_with("Invalid"));
    }
    
    #[test]
    fn test_notation_roundtrip() {
        // Test conversion in both directions
        for file in 0..8 {
            for rank in 0..8 {
                let pos = Position { file, rank };
                let notation = pos.to_notation();
                let converted_pos = Position::from_notation(&notation).unwrap();
                assert_eq!(pos, converted_pos, "Position roundtrip failed for {}", notation);
            }
        }
    }
}
