use super::r#trait::{uiprivate::Render, UI};
use ::crossterm::event::{KeyCode::Char, KeyEvent, KeyModifiers};
use ::yansi::{Color, Paint};

const HERO_MESSAGE: &str = include_str!("../config/hero_message.txt");

pub struct Menu {}

impl Menu {
    pub fn new() -> Menu {
        Menu {}
    }
}

impl Render for Menu {
    fn render(&self) -> String {
        println!("{}", HERO_MESSAGE);
        Paint::new("").fg(Color::Red).to_string()
    }
}

impl UI for Menu {
    fn key(&mut self, event: KeyEvent) -> bool {
        let result = match event {
            KeyEvent {
                code: Char('q'),
                modifiers: KeyModifiers::ALT,
            } => false,
            _ => true,
        };
        println!("{}", self.render());
        result
    }
}
