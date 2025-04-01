use serde::{Deserialize, Serialize};
use crate::rules::{Board, Position, Color, PieceType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promotion: Option<PieceType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub current_player: Color,
    moves: Vec<Move>,
    pieces_moved: Vec<Position>, // Tracks which pieces have moved (for castling)
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Board::new_game(),
            current_player: Color::White,
            moves: Vec::new(),
            pieces_moved: Vec::new(),
        }
    }
    
    pub fn switch_turn(&mut self) {
        self.current_player = self.current_player.opposite();
    }
    
    pub fn record_move(&mut self, from: Position, to: Position, promotion: Option<PieceType>) {
        self.moves.push(Move { from, to, promotion });
        self.pieces_moved.push(from);
    }
    
    pub fn get_last_move(&self) -> Option<&Move> {
        self.moves.last()
    }
    
    pub fn has_piece_moved(&self, pos: Position) -> bool {
        self.pieces_moved.contains(&pos)
    }
    
    pub fn is_game_over(&self) -> bool {
        // Check for checkmate
        if self.board.is_checkmate(self.current_player, self) {
            return true;
        }
        
        // Check for stalemate
        if self.board.is_stalemate(self.current_player, self) {
            return true;
        }
        
        // TODO: Add other draw conditions (insufficient material, threefold repetition, fifty-move rule)
        false
    }
    
    pub fn get_game_result(&self) -> Option<GameResult> {
        if !self.is_game_over() {
            return None;
        }
        
        if self.board.is_checkmate(self.current_player, self) {
            Some(GameResult::Checkmate(self.current_player.opposite()))
        } else if self.board.is_stalemate(self.current_player, self) {
            Some(GameResult::Stalemate)
        } else {
            None // Other draw conditions not yet implemented
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Checkmate(Color), // Winner
    Stalemate,
    // TODO: Add other draw conditions
}

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::rules::{Board, Color, Position, Piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promotion_piece: Option<Piece>,
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
    
    // Track captured pieces
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
    pub fn record_move(&mut self, from: Position, to: Position, promotion_piece: Option<Piece>) {
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
        
        // If this was a capture, record the captured piece
        if let Some(captured_piece) = self.board.get_piece(to) {
            self.captured_pieces.push(captured_piece);
        }
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
}


use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;
use crate::rules::{Color, PieceType, Piece};

// Embed SVG files at compile time
const WHITE_PAWN: &[u8] = include_bytes!("../../assets/white_pawn.svg");
const WHITE_KNIGHT: &[u8] = include_bytes!("../../assets/white_knight.svg");
const WHITE_BISHOP: &[u8] = include_bytes!("../../assets/white_bishop.svg");
const WHITE_ROOK: &[u8] = include_bytes!("../../assets/white_rook.svg");
const WHITE_QUEEN: &[u8] = include_bytes!("../../assets/white_queen.svg");
const WHITE_KING: &[u8] = include_bytes!("../../assets/white_king.svg");

const BLACK_PAWN: &[u8] = include_bytes!("../../assets/black_pawn.svg");
const BLACK_KNIGHT: &[u8] = include_bytes!("../../assets/black_knight.svg");
const BLACK_BISHOP: &[u8] = include_bytes!("../../assets/black_bishop.svg");
const BLACK_ROOK: &[u8] = include_bytes!("../../assets/black_rook.svg");
const BLACK_QUEEN: &[u8] = include_bytes!("../../assets/black_queen.svg");
const BLACK_KING: &[u8] = include_bytes!("../../assets/black_king.svg");

lazy_static! {
        static ref PIECE_SPRITES: HashMap<(Color, PieceType), Arc<svg::Handle>> = {
                let mut sprites = HashMap::new();
                
                // White pieces
                sprites.insert(
                        (Color::White, PieceType::Pawn),
                        Arc::new(svg::Handle::from_memory(WHITE_PAWN))
                    );
                sprites.insert(
                        (Color::White, PieceType::Knight),
                        Arc::new(svg::Handle::from_memory(WHITE_KNIGHT))
                    );
                sprites.insert(
                        (Color::White, PieceType::Bishop),
                        Arc::new(svg::Handle::from_memory(WHITE_BISHOP))
                    );
                sprites.insert(
                        (Color::White, PieceType::Rook),
                        Arc::new(svg::Handle::from_memory(WHITE_ROOK))
                    );
                sprites.insert(
                        (Color::White, PieceType::Queen),
                        Arc::new(svg::Handle::from_memory(WHITE_QUEEN))
                    );
                sprites.insert(
                        (Color::White, PieceType::King),
                        Arc::new(svg::Handle::from_memory(WHITE_KING))
                    );
                
                // Black pieces
                sprites.insert(
                        (Color::Black, PieceType::Pawn),
                        Arc::new(svg::Handle::from_memory(BLACK_PAWN))
                    );
                sprites.insert(
                        (Color::Black, PieceType::Knight),
                        Arc::new(svg::Handle::from_memory(BLACK_KNIGHT))
                    );
                sprites.insert(
                        (Color::Black, PieceType::Bishop),
                        Arc::new(svg::Handle::from_memory(BLACK_BISHOP))
                    );
                sprites.insert(
                        (Color::Black, PieceType::Rook),
                        Arc::new(svg::Handle::from_memory(BLACK_ROOK))
                    );
                sprites.insert(
                        (Color::Black, PieceType::Queen),
                        Arc::new(svg::Handle::from_memory(BLACK_QUEEN))
                    );
                sprites.insert(
                        (Color::Black, PieceType::King),
                        Arc::new(svg::Handle::from_memory(BLACK_KING))
                    );
                
                sprites
            };
}

    pub struct PieceSprite;

    impl PieceSprite {
        /// Get the SVG handle for a specific chess piece
    pub fn get(piece: &Piece) -> Arc<svg::Handle> {
                PIECE_SPRITES
                    .get(&(piece.color, piece.piece_type))
                        .expect("Sprite should exist for every piece type")
                        .clone()
                }
        
        /// Create an Svg widget for a specific chess piece
    pub fn widget(piece: &Piece) -> Svg {
                Svg::new(Self::get(piece).as_ref().clone())
            }
}

    }
    }
}
        )
        )
        )
        )
        )
        )
        )
        )
        )
        )
        )
        )
    }
}
