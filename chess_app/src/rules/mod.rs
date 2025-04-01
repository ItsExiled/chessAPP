use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub rank: i8,
    pub file: i8,
}

impl Position {
    pub fn is_valid(&self) -> bool {
        self.rank >= 0 && self.rank < 8 && self.file >= 0 && self.file < 8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pieces: HashMap<Position, Piece>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pieces: HashMap::new(),
        }
    }
    
    pub fn new_game() -> Self {
        let mut board = Board::new();
        
        // Place pieces for both colors
        for color in [Color::White, Color::Black] {
            let home_rank = if color == Color::White { 0 } else { 7 };
            let pawn_rank = if color == Color::White { 1 } else { 6 };
            
            // Place pawns
            for file in 0..8 {
                board.set_piece(
                    Position { rank: pawn_rank, file },
                    Some(Piece { piece_type: PieceType::Pawn, color }),
                );
            }
            
            // Place other pieces
            let pieces = [
                PieceType::Rook,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Queen,
                PieceType::King,
                PieceType::Bishop,
                PieceType::Knight,
                PieceType::Rook,
            ];
            
            for (file, &piece_type) in pieces.iter().enumerate() {
                board.set_piece(
                    Position { rank: home_rank, file: file as i8 },
                    Some(Piece { piece_type, color }),
                );
            }
        }
        
        board
    }
    
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        self.pieces.get(&pos).copied()
    }
    
    pub fn set_piece(&mut self, pos: Position, piece: Option<Piece>) {
        match piece {
            Some(p) => { self.pieces.insert(pos, p); }
            None => { self.pieces.remove(&pos); }
        }
    }
    
    pub fn is_in_check(&self, color: Color) -> bool {
        // Find the king
        let king_pos = self.pieces.iter()
            .find(|(_, piece)| piece.piece_type == PieceType::King && piece.color == color)
            .map(|(pos, _)| *pos);
        
        if let Some(king_pos) = king_pos {
            // Check if any opponent piece can attack the king
            for (pos, piece) in self.pieces.iter() {
                if piece.color == color.opposite() {
                    if self.is_valid_attack(*pos, king_pos, piece.color) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    fn is_valid_attack(&self, from: Position, to: Position, color: Color) -> bool {
        if !from.is_valid() || !to.is_valid() {
            return false;
        }
        
        let piece = match self.get_piece(from) {
            Some(p) => p,
            None => return false,
        };
        
        if piece.color != color {
            return false;
        }
        
        match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_attack(from, to, color),
            PieceType::Knight => self.is_valid_knight_move(from, to),
            PieceType::Bishop => self.is_valid_bishop_move(from, to),
            PieceType::Rook => self.is_valid_rook_move(from, to),
            PieceType::Queen => self.is_valid_queen_move(from, to),
            PieceType::King => self.is_valid_king_move(from, to),
        }
    }
    
    fn is_valid_pawn_attack(&self, from: Position, to: Position, color: Color) -> bool {
        let direction = if color == Color::White { 1 } else { -1 };
        let rank_diff = to.rank - from.rank;
        let file_diff = (to.file - from.file).abs();
        
        rank_diff == direction && file_diff == 1
    }
    
    fn is_valid_knight_move(&self, from: Position, to: Position) -> bool {
        let rank_diff = (to.rank - from.rank).abs();
        let file_diff = (to.file - from.file).abs();
        
        (rank_diff == 2 && file_diff == 1) || (rank_diff == 1 && file_diff == 2)
    }
    
    fn is_valid_bishop_move(&self, from: Position, to: Position) -> bool {
        let rank_diff = (to.rank - from.rank).abs();
        let file_diff = (to.file - from.file).abs();
        
        if rank_diff != file_diff {
            return false;
        }
        
        // Check if path is clear
        let rank_step = (to.rank - from.rank).signum();
        let file_step = (to.file - from.file).signum();
        
        let mut current = Position {
            rank: from.rank + rank_step,
            file: from.file + file_step,
        };
        
        while current != to {
            if self.get_piece(current).is_some() {
                return false;
            }
            current.rank += rank_step;
            current.file += file_step;
        }
        
        true
    }
    
    fn is_valid_rook_move(&self, from: Position, to: Position) -> bool {
        if from.rank != to.rank && from.file != to.file {
            return false;
        }
        
        // Check if path is clear
        let rank_step = (to.rank - from.rank).signum();
        let file_step = (to.file - from.file).signum();
        
        let mut current = Position {
            rank: from.rank + rank_step,
            file: from.file + file_step,
        };
        
        while current != to {
            if self.get_piece(current).is_some() {
                return false;
            }
            current.rank += rank_step;
            current.file += file_step;
        }
        
        true
    }
    
    fn is_valid_queen_move(&self, from: Position, to: Position) -> bool {
        self.is_valid_bishop_move(from, to) || self.is_valid_rook_move(from, to)
    }
    
    fn is_valid_king_move(&self, from: Position, to: Position) -> bool {
        let rank_diff = (to.rank - from.rank).abs();
        let file_diff = (to.file - from.file).abs();
        
        rank_diff <= 1 && file_diff <= 1
    }
}

use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub rank: i8,
    pub file: i8,
}

impl Position {
    pub fn is_valid(&self) -> bool {
        self.rank >= 0 && self.rank < 8 && self.file >= 0 && self.file < 8
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: [[None; 8]; 8],
        }
    }
    
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if !pos.is_valid() {
            return None;
        }
        self.squares[pos.rank as usize][pos.file as usize]
    }
    
    pub fn set_piece(&mut self, pos: Position, piece: Option<Piece>) {
        if pos.is_valid() {
            self.squares[pos.rank as usize][pos.file as usize] = piece;
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub rank: i8,  // 0-7 (1-8 in chess notation)
    pub file: i8,  // 0-7 (a-h in chess notation)
}

impl Position {
    pub fn new(file: i8, rank: i8) -> Option<Self> {
        if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
            Some(Position { rank, file })
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        self.file >= 0 && self.file < 8 && self.rank >= 0 && self.rank < 8
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        // Initialize an empty board
        Board {
            pieces: [[None; 8]; 8],
        }
    }

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

    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.is_valid() {
            self.pieces[pos.rank as usize][pos.file as usize]
        } else {
            None
        }
    }

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
    
    fn is_valid_knight_move(&self, from: Position, to: Position) -> bool {
        let rank_diff = self.rank_distance(from, to);
        let file_diff = self.file_distance(from, to);
        
        // Knight moves in an L-shape: 2 squares in one direction and 1 square perpendicular
        (rank_diff == 2 && file_diff == 1) || (rank_diff == 1 && file_diff == 2)
    }
    
    fn is_valid_bishop_move(&self, from: Position, to: Position) -> bool {
        // Bishop moves diagonally
        if !self.is_diagonal_move(from, to) {
            return false;
        }
        
        // Check if the path is clear
        self.is_path_clear(from, to)
    }
    
    fn is_valid_rook_move(&self, from: Position, to: Position) -> bool {
        // Rook moves in straight lines
        if !self.is_straight_move(from, to) {
            return false;
        }
        
        // Check if the path is clear
        self.is_path_clear(from, to)
    }
    
    fn is_valid_queen_move(&self, from: Position, to: Position) -> bool {
        // Queen can move like a bishop or rook
        if !self.is_diagonal_move(from, to) && !self.is_straight_move(from, to) {
            return false;
        }
        
        // Check if the path is clear
        self.is_path_clear(from, to)
    }
    
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
                (7, 5..7, vec![Position { rank: king_rank, file: 5 }, Position { rank: king_rank, file: 6 }])
            } else if to.file == 2 {
                // Queenside castling
                (0, 1..4, vec![Position { rank: king_rank, file: 3 }, Position { rank: king_rank, file: 2 }])
            } else {
                return false;
            };
            
            // Check if rook has moved
            let rook_pos = Position { rank: king_rank, file: rook_file };
            if game_state.has_piece_moved(rook_pos) {
                return false;
            }
            
            // Check if rook is present
            match self.get_piece(rook_pos) {
                Some(piece) => {
                    if piece.piece_type != PieceType::Rook || piece.color != color {
                        return false;
                    }
                },
                None => return false,
            }
            
            // Check if path is clear
            for file in path_range {
                let pos = Position { rank: king_rank, file };
                if self.get_piece(pos).is_some() {
                    return false;
                }
            }
            
            // Check if king is currently in check
            if self.is_in_check(color) {
                return false;
            }
            
            // Check if any square the king moves through is under attack
            for square in through_squares {
                if self.is_square_attacked(square, color.opposite()) {
                    return false;
                }
            }
            
            return true;
        }
        
        false
    }
    
    /// Check if a square is under attack by a specific color
    /// Check if a square is under attack by a specific color
    pub fn is_square_attacked(&self, pos: Position, by_color: Color) -> bool {
        // Check all opponent's pieces for potential attacks
        for rank in 0..8 {
            for file in 0..8 {
                let from = Position { rank, file };
                if let Some(piece) = self.get_piece(from) {
                    if piece.color == by_color {
                        // For all pieces except king (to avoid infinite recursion)
                        if piece.piece_type != PieceType::King {
                            // We need to avoid infinite recursion, so we don't use is_valid_move
                            let valid = match piece.piece_type {
                                PieceType::Pawn => {
                                    // Manual check for pawn attacks (diagonal only)
                                    let forward = match piece.color {
                                        Color::White => 1,
                                        Color::Black => -1,
                                    };
                                    let rank_diff = pos.rank - from.rank;
                                    let file_diff = self.file_distance(from, pos);
                                    rank_diff == forward && file_diff == 1
                                },
                                PieceType::Knight => self.is_valid_knight_move(from, pos),
                                PieceType::Bishop => self.is_valid_bishop_move(from, pos),
                                PieceType::Rook => self.is_valid_rook_move(from, pos),
                                PieceType::Queen => self.is_valid_queen_move(from, pos),
                                _ => false, // Should not reach here
                            };
                            
                            if valid {
                                return true;
                            }
                        } else {
                            // Manual check for king (one square in any direction)
                            let rank_diff = self.rank_distance(from, pos);
                            let file_diff = self.file_distance(from, pos);
                            if rank_diff <= 1 && file_diff <= 1 {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
    
    /// Checks if the given color is in check
    pub fn is_in_check(&self, color: Color) -> bool {
        // Find the king
        let mut king_pos = None;
        for rank in 0..8 {
            for file in 0..8 {
                let pos = Position { rank, file };
                if let Some(piece) = self.get_piece(pos) {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        king_pos = Some(pos);
                        break;
                    }
                }
            }
            if king_pos.is_some() {
                break;
            }
        }
        
        if let Some(king_pos) = king_pos {
            self.is_square_attacked(king_pos, color.opposite())
        } else {
            false // This shouldn't happen in a valid game
        }
    }
    
    /// Returns true if the given color is in checkmate
    pub fn is_checkmate(&self, color: Color, game_state: &GameState) -> bool {
        // If the king is not in check, it's not checkmate
        if !self.is_in_check(color) {
            return false;
        }
        
        // If there are no legal moves, it's checkmate
        !self.has_legal_moves(color, game_state)
    }
    
    /// Returns true if the given color is in stalemate (not in check but no legal moves)
    pub fn is_stalemate(&self, color: Color, game_state: &GameState) -> bool {
        // If the king is in check, it's not stalemate
        if self.is_in_check(color) {
            return false;
        }
        
        // If there are no legal moves, it's stalemate
        !self.has_legal_moves(color, game_state)
    }
    
    /// Returns true if the given color has any legal moves available
    fn has_legal_moves(&self, color: Color, game_state: &GameState) -> bool {
        // Check all pieces of the given color
        for rank in 0..8 {
            for file in 0..8 {
                let from = Position { rank, file };
                if let Some(piece) = self.get_piece(from) {
                    if piece.color == color {
                        // Check all possible destination squares
                        for to_rank in 0..8 {
                            for to_file in 0..8 {
                                let to = Position { rank: to_rank, file: to_file };
                                // Create a temporary board to test the move
                                if self.is_valid_move(from, to, color, game_state) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::GameState;

    #[test]
    fn test_new_game_board_setup() {
        let board = Board::new_game();
        
        // Test white pieces
        assert_eq!(
            board.get_piece(Position { rank: 0, file: 0 }),
            Some(Piece { piece_type: PieceType::Rook, color: Color::White })
        );
        assert_eq!(
            board.get_piece(Position { rank: 0, file: 1 }),
            Some(Piece { piece_type: PieceType::Knight, color: Color::White })
        );
        assert_eq!(
            board.get_piece(Position { rank: 0, file: 2 }),
            Some(Piece { piece_type: PieceType::Bishop, color: Color::White })
        );
        assert_eq!(
            board.get_piece(Position { rank: 0, file: 3 }),
            Some(Piece { piece_type: PieceType::Queen, color: Color::White })
        );
        assert_eq!(
            board.get_piece(Position { rank: 0, file: 4 }),
            Some(Piece { piece_type: PieceType::King, color: Color::White })
        );
        
        // Test black pieces
        assert_eq!(
            board.get_piece(Position { rank: 7, file: 0 }),
            Some(Piece { piece_type: PieceType::Rook, color: Color::Black })
        );
        assert_eq!(
            board.get_piece(Position { rank: 7, file: 4 }),
            Some(Piece { piece_type: PieceType::King, color: Color::Black })
        );
        
        // Test pawns
        for file in 0..8 {
            assert_eq!(
                board.get_piece(Position { rank: 1, file }),
                Some(Piece { piece_type: PieceType::Pawn, color: Color::White })
            );
            assert_eq!(
                board.get_piece(Position { rank: 6, file }),
                Some(Piece { piece_type: PieceType::Pawn, color: Color::Black })
            );
        }
        
        // Test empty squares
        for rank in 2..6 {
            for file in 0..8 {
                assert_eq!(board.get_piece(Position { rank, file }), None);
            }
        }
    }
    
    #[test]
    fn test_pawn_moves() {
        let mut board = Board::new_game();
        let game_state = GameState::new();
        
        // Test white pawn initial double move
        assert!(board.is_valid_move(
            Position { rank: 1, file: 0 },
            Position { rank: 3, file: 0 },
            Color::White,
            &game_state
        ));
        
        // Test white pawn single move
        assert!(board.is_valid_move(
            Position { rank: 1, file: 0 },
            Position { rank: 2, file: 0 },
            Color::White,
            &game_state
        ));
        
        // Test invalid backward move
        assert!(!board.is_valid_move(
            Position { rank: 1, file: 0 },
            Position { rank: 0, file: 0 },
            Color::White,
            &game_state
        ));
        
        // Test diagonal capture
        board.set_piece(
            Position { rank: 2, file: 1 },
            Some(Piece { piece_type: PieceType::Pawn, color: Color::Black })
        );
        assert!(board.is_valid_move(
            Position { rank: 1, file: 0 },
            Position { rank: 2, file: 1 },
            Color::White,
            &game_state
        ));
    }
    
    #[test]
    fn test_knight_moves() {
        let board = Board::new_game();
        let game_state = GameState::new();
        
        // Test valid L-shaped moves
        assert!(board.is_valid_move(
            Position { rank: 0, file: 1 },
            Position { rank: 2, file: 0 },
            Color::White,
            &game_state
        ));
        
        assert!(board.is_valid_move(
            Position { rank: 0, file: 1 },
            Position { rank: 2, file: 2 },
            Color::White,
            &game_state
        ));
        
        // Test invalid move
        assert!(!board.is_valid_move(
            Position { rank: 0, file: 1 },
            Position { rank: 2, file: 3 },
            Color::White,
            &game_state
        ));
    }
    
    #[test]
    fn test_king_in_check() {
        let mut board = Board::new();
        
        // Set up a simple check position
        board.set_piece(
            Position { rank: 0, file: 4 },
            Some(Piece { piece_type: PieceType::King, color: Color::White })
        );
        board.set_piece(
            Position { rank: 7, file: 4 },
            Some(Piece { piece_type: PieceType::Rook, color: Color::Black })
        );
        
        assert!(board.is_in_check(Color::White));
        assert!(!board.is_in_check(Color::Black));
    }
    
    #[test]
    fn test_castling() {
        let mut game_state = GameState::new();
        
        // Clear pieces between king and rook
        game_state.board.set_piece(Position { rank: 0, file: 5 }, None);
        game_state.board.set_piece(Position { rank: 0, file: 6 }, None);
        
        // Test kingside castling
        assert!(game_state.board.is_valid_move(
            Position { rank: 0, file: 4 },
            Position { rank: 0, file: 6 },
            Color::White,
            &game_state
        ));
        
        // Test invalid castling after king has moved
        game_state.record_move(
            Position { rank: 0, file: 4 },
            Position { rank: 0, file: 5 },
            None
        );
        game_state.record_move(
            Position { rank: 0, file: 5 },
            Position { rank: 0, file: 4 },
            None
        );
        
        assert!(!game_state.board.is_valid_move(
            Position { rank: 0, file: 4 },
            Position { rank: 0, file: 6 },
            Color::White,
            &game_state
        ));
    }
}
