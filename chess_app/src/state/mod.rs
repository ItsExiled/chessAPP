use serde::{Deserialize, Serialize};
use crate::board::Board;
use crate::types::{Color, Piece, PieceType, Position};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promotion_piece: Option<PieceType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    InProgress,
    Check { player: Color },
    Checkmate { winner: Color },
    Stalemate,
    Draw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub current_player: Color,
    pub status: GameStatus,
    
    // Track number of moves for each piece (for castling eligibility)
    piece_move_history: HashMap<Position, u32>,
    
    // Track the last move (for en passant)
    last_move: Option<Move>,
    
    // Track promoted pawns
    promoted_pawns: HashSet<Position>,
    
    // Track move history for threefold repetition
    move_history: Vec<Move>,
    
    // Track captured assets
    captured_pieces: Vec<Piece>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Board::new_game(),
            current_player: Color::White,
            status: GameStatus::InProgress,
            piece_move_history: HashMap::new(),
            last_move: None,
            promoted_pawns: HashSet::new(),
            move_history: Vec::new(),
            captured_pieces: Vec::new(),
        }
    }
    
    /// Record a piece movement
    pub fn record_move(&mut self, from: Position, to: Position, promotion_piece: Option<PieceType>) {
        let move_count = self.piece_move_history.entry(from).or_insert(0);
        *move_count += 1;
        
        let game_move = Move {
            from,
            to,
            promotion_piece,
        };
        
        // Record last move for en passant detection
        self.last_move = Some(game_move.clone());
        
        // Record move in history for threefold repetition detection
        self.move_history.push(game_move);
    }
    
    /// Check if a piece has moved (for castling)
    pub fn has_piece_moved(&self, pos: Position) -> bool {
        self.piece_move_history.get(&pos).copied().unwrap_or(0) > 0
    }
    
    /// Get the last move (for en passant)
    pub fn get_last_move(&self) -> Option<&Move> {
        self.last_move.as_ref()
    }
    
    /// Record a pawn promotion
    pub fn record_promotion(&mut self, pos: Position) {
        self.promoted_pawns.insert(pos);
    }
    
    /// Check if a pawn has been promoted
    pub fn is_promoted_pawn(&self, pos: Position) -> bool {
        self.promoted_pawns.contains(&pos)
    }
    
    /// Switch the current player
    pub fn switch_turn(&mut self) {
        self.current_player = match self.current_player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }
    
    /// Update the game status
    pub fn update_status(&mut self, new_status: GameStatus) {
        self.status = new_status;
    }

    /// Get a Unicode character representation of a piece
    pub fn get_piece_symbol(piece: &Piece) -> &'static str {
        match (piece.piece_type, piece.color) {
            (PieceType::King, Color::White) => "♔",
            (PieceType::Queen, Color::White) => "♕",
            (PieceType::Rook, Color::White) => "♖",
            (PieceType::Bishop, Color::White) => "♗",
            (PieceType::Knight, Color::White) => "♘",
            (PieceType::Pawn, Color::White) => "♙",
            (PieceType::King, Color::Black) => "♚",
            (PieceType::Queen, Color::Black) => "♛",
            (PieceType::Rook, Color::Black) => "♜",
            (PieceType::Bishop, Color::Black) => "♝",
            (PieceType::Knight, Color::Black) => "♞",
            (PieceType::Pawn, Color::Black) => "♟",
        }
    }
}
