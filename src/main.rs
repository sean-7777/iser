mod ui;

use ::crossterm::{
    event::{self, Event},
    terminal,
};
use ui::UI;

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut renderable = ui::menu::Menu::new();
    loop {
        if let Event::Key(evt) = event::read().unwrap() {
            if !renderable.key(evt) {
                println!("ended");
                break;
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
}
