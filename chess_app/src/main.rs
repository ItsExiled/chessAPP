mod gui;
mod rules;
mod state;
mod assets;

use iced::{
    Application, Command, Element, Settings,
    window, executor,
};
use gui::{ChessGui, Message};

pub struct ChessApp {
    gui: ChessGui,
}

impl Application for ChessApp {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ChessApp, Command<Message>) {
        (
            ChessApp {
                gui: ChessGui::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Chess Game")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.gui.update(message);
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        self.gui.view()
    }
}

pub fn main() -> iced::Result {
    ChessApp::run(Settings {
        window: window::Settings {
            size: (500, 500),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}

use iced::{
    Sandbox, Settings, window,
    widget::container,
    Element,
};

mod assets;
mod gui;
mod rules;
mod state;

use gui::{ChessGui, Message};

pub struct ChessApp {
    gui: ChessGui,
}

impl Sandbox for ChessApp {
    type Message = Message;
    
    fn new() -> Self {
        ChessApp {
            gui: ChessGui::new(),
        }
    }
    
    fn title(&self) -> String {
        String::from("Chess Game")
    }
    
    fn update(&mut self, message: Message) {
        self.gui.update(message);
    }
    
    fn view(&self) -> Element<Message> {
        self.gui.view()
    }
}

fn main() -> iced::Result {
    ChessApp::run(Settings {
        window: window::Settings {
            size: (600, 600),
            ..Default::default()
        },
        ..Default::default()
    })
}

fn main() {
    println!("Hello, world!");
}
