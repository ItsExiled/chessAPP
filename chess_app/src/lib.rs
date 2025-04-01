//! # Chess Application Core Library
//!
//! This crate provides the core functionality for a chess application, including:
//!
//! - Chess piece types and board representation
//! - Move validation and game rules
//! - Game state management
//!
//! ## Module Structure
//!
//! - `types`: Core type definitions (Color, PieceType, Piece, Position)
//! - `board`: Chess board implementation with move validation
//! - `state`: Game state management and turn tracking
//!
//! ## Usage
//!
//! The main types are re-exported at the crate root for convenience.
//! Most applications will use the `Board` and `GameState` types to manage a chess game.
//!
//! ```rust,no_run
//! use chess_app::{Board, GameState, Position, Color};
//!
//! let mut board = Board::new_game();
//! let mut game = GameState::new();
//! 
//! // Make a move from e2 to e4
//! let from = Position::new(4, 1);
//! let to = Position::new(4, 3);
//! 
//! if board.make_move(&from, &to) {
//!     println!("Moved piece from e2 to e4");
//! }
//! ```

// Public modules
pub mod types;
pub mod board;
pub mod state;

// Test configuration
#[cfg(test)]
mod tests;

// Re-export common types for easier access
pub use types::{Color, Piece, PieceType, Position};
pub use board::Board;
pub use state::GameState;
