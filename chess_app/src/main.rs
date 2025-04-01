mod gui;
mod state;
mod rules;
mod ai;

use iced::{
    Application, Command, Element, Settings, Theme,
    executor, window,
};

use gui::{GuiState, GuiMessage, Screen};
use state::GameState;

pub struct ChessApp {
    gui_state: GuiState,
    game_state: Option<GameState>,
}

impl Application for ChessApp {
    type Message = GuiMessage;
    type Theme = Theme;
    type Executor = executor::Default;
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
        }
        Command::none()
    }

    fn view(&self) -> Element<GuiMessage> {
        self.gui_state.view()
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

mod gui;
mod state;
mod rules;
mod ai;

use iced::{
    Application, Command, Element, Settings, Theme,
    executor, window,
};

pub struct ChessApp {
    // We'll add state here later
}

#[derive(Debug, Clone)]
pub enum Message {
    // We'll add messages here later
}

impl Application for ChessApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ChessApp, Command<Message>) {
        (
            ChessApp {
                // Initialize state here later
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Chess Game")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // We'll implement the view later
        iced::widget::text("Chess Game").into()
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

