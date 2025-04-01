mod gui;
mod state;
mod board;
mod types;
// Comment out the rules module which duplicates types
// mod rules;
mod ai;

use ai::ChessAI;
use types::Color;
use iced::{
    executor, window, Application, Element, Settings, Theme,
    Command,
};

use gui::{GuiState, GuiMessage, Screen};
use state::GameState;

pub struct ChessApp {
    gui_state: GuiState,
    game_state: Option<GameState>,
    chess_ai: Option<ChessAI>,
}

impl Application for ChessApp {
    type Executor = executor::Default;
    type Message = GuiMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (ChessApp, Command<GuiMessage>) {
        (
            ChessApp {
                gui_state: GuiState::new(),
                game_state: None,
                chess_ai: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Chess Game")
    }

    fn update(&mut self, message: GuiMessage) -> Command<GuiMessage> {
        match message {
            GuiMessage::NewGame => {
                self.game_state = Some(GameState::new());
                self.gui_state.screen = Screen::Game;
                self.chess_ai = Some(ChessAI::new(Color::Black, self.gui_state.selected_difficulty.clone()));
            }
            GuiMessage::SetDifficulty(difficulty) => {
                self.gui_state.selected_difficulty = difficulty;
            }
            GuiMessage::LoadGame => {
                // TODO: Implement game loading
            }
            GuiMessage::BackToMenu => {
                self.gui_state.screen = Screen::MainMenu;
                self.game_state = None;
                self.chess_ai = None;
            }
            GuiMessage::SquareSelected(pos) => {
                // Handle square selection for moves
                if let Some(selected) = self.gui_state.selected_square {
                    // Attempt to make a move if a square was already selected
                    if let Some(game_state) = &mut self.game_state {
                        if game_state.board.is_valid_move(&selected, &pos) {
                            game_state.board.make_move(&selected, &pos);
                            game_state.switch_turn();
                            
                            // If it's now the AI's turn (Black), make an AI move
                            if game_state.current_player == Color::Black {
                                if let Some(chess_ai) = &self.chess_ai {
                                    if let Some((from, to)) = chess_ai.get_best_move(game_state) {
                                        // Make the AI's move
                                        if game_state.board.is_valid_move(&from, &to) {
                                            game_state.board.make_move(&from, &to);
                                            game_state.switch_turn(); // Switch back to player's turn
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self.gui_state.selected_square = None;
                } else {
                    // Select the square if it contains a piece of the current player
                    if let Some(game_state) = &self.game_state {
                        if let Some(piece) = game_state.board.get_piece(&pos) {
                            if piece.color == game_state.current_player {
                                self.gui_state.selected_square = Some(pos);
                            }
                        }
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<GuiMessage> {
        self.gui_state.view(self.game_state.as_ref())
    }
}

pub fn main() -> iced::Result {
    // Create assets directory if it doesn't exist
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/home/exiled"));
    let assets_dir = format!("{}/chessAPP/chess_app/assets", home);
    std::fs::create_dir_all(&assets_dir).ok();
    
    ChessApp::run(Settings {
        window: window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        antialiasing: true,  // Enable antialiasing for better text rendering
        ..Default::default()
    })
}
