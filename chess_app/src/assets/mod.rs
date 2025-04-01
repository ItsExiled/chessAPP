use std::path::PathBuf;
use iced::widget::{svg, container, text};
use iced::{Element, Length};
use crate::rules::{Piece, Color, PieceType};

pub struct PieceSprite;

impl PieceSprite {
    /// Get the path to the SVG file for a given piece
    fn get_path(piece: &Piece) -> PathBuf {
        let color = match piece.color {
            Color::White => "white",
            Color::Black => "black",
        };
        
        let piece_name = match piece.piece_type {
            PieceType::Pawn => "pawn",
            PieceType::Knight => "knight",
            PieceType::Bishop => "bishop",
            PieceType::Rook => "rook",
            PieceType::Queen => "queen",
            PieceType::King => "king",
        };
        
        PathBuf::from(format!("assets/{}_{}.svg", color, piece_name))
    }
    
    /// Create a widget to display the piece
    pub fn widget<'a, Message>(piece: &Piece) -> Element<'a, Message> {
        let path = Self::get_path(piece);
        
        match svg::Handle::from_path(path) {
            Ok(handle) => {
                container(
                    svg(handle)
                        .width(Length::Fixed(50.0))
                        .height(Length::Fixed(50.0))
                )
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0))
                .center_x()
                .center_y()
                .into()
            }
            Err(_) => {
                // Fallback to text representation if SVG loading fails
                let symbol = match (piece.color, piece.piece_type) {
                    (Color::White, PieceType::Pawn) => "♙",
                    (Color::White, PieceType::Knight) => "♘",
                    (Color::White, PieceType::Bishop) => "♗",
                    (Color::White, PieceType::Rook) => "♖",
                    (Color::White, PieceType::Queen) => "♕",
                    (Color::White, PieceType::King) => "♔",
                    (Color::Black, PieceType::Pawn) => "♟",
                    (Color::Black, PieceType::Knight) => "♞",
                    (Color::Black, PieceType::Bishop) => "♝",
                    (Color::Black, PieceType::Rook) => "♜",
                    (Color::Black, PieceType::Queen) => "♛",
                    (Color::Black, PieceType::King) => "♚",
                };
                
                container(text(symbol).size(40))
                    .width(Length::Fixed(60.0))
                    .height(Length::Fixed(60.0))
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}

use iced::widget::svg;
use lazy_static::lazy_static;
use crate::rules::{Piece, Color, PieceType};

lazy_static! {
    static ref WHITE_PAWN: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/white_pawn.svg"));
    static ref WHITE_KNIGHT: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/white_knight.svg"));
    static ref WHITE_BISHOP: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/white_bishop.svg"));
    static ref WHITE_ROOK: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/white_rook.svg"));
    static ref WHITE_QUEEN: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/white_queen.svg"));
    static ref WHITE_KING: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/white_king.svg"));
    
    static ref BLACK_PAWN: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/black_pawn.svg"));
    static ref BLACK_KNIGHT: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/black_knight.svg"));
    static ref BLACK_BISHOP: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/black_bishop.svg"));
    static ref BLACK_ROOK: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/black_rook.svg"));
    static ref BLACK_QUEEN: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/black_queen.svg"));
    static ref BLACK_KING: svg::Handle = svg::Handle::from_memory(include_bytes!("../../assets/black_king.svg"));
}

pub struct PieceSprite;

impl PieceSprite {
    pub fn widget(piece: &Piece) -> iced::Element<'static, crate::gui::Message> {
        let handle = match (piece.color, piece.piece_type) {
            (Color::White, PieceType::Pawn) => WHITE_PAWN.clone(),
            (Color::White, PieceType::Knight) => WHITE_KNIGHT.clone(),
            (Color::White, PieceType::Bishop) => WHITE_BISHOP.clone(),
            (Color::White, PieceType::Rook) => WHITE_ROOK.clone(),
            (Color::White, PieceType::Queen) => WHITE_QUEEN.clone(),
            (Color::White, PieceType::King) => WHITE_KING.clone(),
            
            (Color::Black, PieceType::Pawn) => BLACK_PAWN.clone(),
            (Color::Black, PieceType::Knight) => BLACK_KNIGHT.clone(),
            (Color::Black, PieceType::Bishop) => BLACK_BISHOP.clone(),
            (Color::Black, PieceType::Rook) => BLACK_ROOK.clone(),
            (Color::Black, PieceType::Queen) => BLACK_QUEEN.clone(),
            (Color::Black, PieceType::King) => BLACK_KING.clone(),
        };
        
        svg::Svg::new(handle)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

