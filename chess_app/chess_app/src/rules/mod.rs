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
}

// Add helper methods for move validation
impl Board {
    /// Validates if a move from one position to another is legal according to chess rules
    pub fn is_valid_move(&self, from: Position, to: Position, current_player: Color) -> bool {
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
        match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(from, to, piece.color),
            PieceType::Knight => self.is_valid_knight_move(from, to),
            PieceType::Bishop => self.is_valid_bishop_move(from, to),
            PieceType::Rook => self.is_valid_rook_move(from, to),
            PieceType::Queen => self.is_valid_queen_move(from, to),
            PieceType::King => self.is_valid_king_move(from, to, piece.color),
        }
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
    
    /// Checks if there are any pieces between two positions (not including the endpoints)
    fn is_path_clear(&self, from: Position, to: Position) -> bool {
        let dx = (to.file - from.file).signum();
        let dy = (to.rank - from.rank).signum();
        
        let mut current = Position {
            file: from.file + dx,
            rank: from.rank + dy,
        };
        
        while current.file != to.file || current.rank != to.rank {
            if self.get_piece(current).is_some() {
                return false;
            }
            current.file += dx;
            current.rank += dy;
        }
        
        true
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
    
    fn is_valid_pawn_move(&self, from: Position, to: Position, color: Color) -> bool {
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
                return self.get_piece(to).is_none();
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
        
        // Diagonal capture
        else if file_diff == 1 && rank_diff == forward_direction {
            // Must capture an opponent's piece
            return self.get_piece(to).is_some();
        }
        
        false
    }
    
    fn is_valid_king_move(&self, from: Position, to: Position, color: Color) -> bool {
        let rank_diff = self.rank_distance(from, to);
        let file_diff = self.file_distance(from, to);
        
        // Regular king move: one square in any direction
        if rank_diff <= 1 && file_diff <= 1 {
            return true;
        }
        
        // Castling
        if rank_diff == 0 && file_diff == 2 {
            let king_rank = match color {
                Color::White => 0,
                Color::Black => 7,
            };
            
            // Ensure king is in correct starting position
            if from.rank != king_rank || from.file != 4 {
                return false;
            }
            
            // Determine if kingside or queenside castling
            let (rook_file, path_range) = if to.file == 6 {
                // Kingside castling
                (7, 5..7)
            } else if to.file == 2 {
                // Queenside castling
                (0, 1..4)
            } else {
                return false;
            };
            
            // Check if rook is in position
            let rook_pos = Position { rank: king_rank, file: rook_file };
            match self.get_piece(rook_pos) {
                Some(piece) => {
                    if piece.piece_type != PieceType::Rook || piece.color != color {
                        return false;
                    }
                }
                None => return false,
            }
            
            // Check if path is clear
            for file in path_range {
                let pos = Position { rank: king_rank, file };
                if self.get_piece(pos).is_some() {
                    return false;
                }
            }
            
            return true;
        }
        
        false
    }
    
    pub fn is_square_attacked(&self, pos: Position, by_color: Color) -> bool {
        // Check all opponent's pieces for potential attacks
        for rank in 0..8 {
            for file in 0..8 {
                let from = Position { rank, file };
                if let Some(piece) = self.get_piece(from) {
                    if piece.color == by_color {
                        // For all pieces except king (to avoid infinite recursion)
                        if piece.piece_type != PieceType::King {
                            if self.is_valid_move(from, pos, by_color) {
                                return true;
                            }
                        } else {
                            // For king, manually check one square radius
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
}
