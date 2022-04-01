use crossterm::event::{KeyCode::Char, KeyEvent, KeyModifiers};

pub struct Test {
    cur: i32,
}

impl Test {
    pub fn new() -> Test {
        Test { cur: 0 }
    }

    fn render(&self) -> String {
        if self.cur == 0 {
            "test".to_string()
        } else {
            "west".to_string()
        }
    }

    pub fn key(&self, event: KeyEvent) -> Option<bool, ()> {
        match event {
            KeyEvent {
                code: Char('h'),
                modifiers: KeyModifiers::NONE,
            } => println!("hi"),
            KeyEvent {
                code: Char('h'),
                modifiers: KeyModifiers::ALT,
            } => println!("HI"),
            KeyEvent {
                code: Char('q'),
                modifiers: KeyModifiers::ALT,
            } => return false,
            _ => (),
        }

        self.render();
    }
}
