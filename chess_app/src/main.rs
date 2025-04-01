mod gui;
mod state;
mod board;
mod types;
mod ai;

use iced::{
    executor, window, Application, Element, Settings, Theme,
    Command,
};

use gui::{GuiState, GuiMessage, Screen};
use state::GameState;

pub struct ChessApp {
    gui_state: GuiState,
    game_state: Option<GameState>,
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
            }
            GuiMessage::SquareSelected(pos) => {
                // Handle square selection for moves
                if let Some(selected) = self.gui_state.selected_square {
                    // Attempt to make a move if a square was already selected
                    if let Some(game_state) = &mut self.game_state {
                        if game_state.board.is_valid_move(&selected, &pos) {
                            game_state.board.make_move(&selected, &pos);
                            game_state.switch_turn();
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
    ChessApp::run(Settings {
        window: window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        ..Default::default()
    })
}
