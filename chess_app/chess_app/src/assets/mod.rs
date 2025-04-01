use iced::widget::svg::{self, Svg};
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

