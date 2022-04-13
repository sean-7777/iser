use ::crossterm::event::KeyEvent;

pub mod uiprivate {
    pub trait Render {
        fn render(&self) -> String;
    }
}

pub trait UI {
    fn key(&mut self, event: KeyEvent) -> bool;
}
