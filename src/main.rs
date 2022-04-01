mod ui;

use crossterm::{cursor, event, style, terminal, ExecutableCommand};
use std::io;
use yansi::{Color, Paint};

const CLEAR: terminal::Clear = terminal::Clear(terminal::ClearType::All);
const MOVE_START: cursor::MoveTo = cursor::MoveTo(0, 0);

fn print_welcome(stdout: &mut io::Stdout) {
    stdout
        .execute(CLEAR)
        .unwrap()
        .execute(MOVE_START)
        .unwrap()
        .execute(style::Print(format!(
            "{}",
            Paint::new("Welcome to this app!")
                .bold()
                .bg(Color::Blue)
                .fg(Color::Green)
        )))
        .unwrap();
}

fn detect_keypresses(stdout: &mut io::Stdout) {
    let renderable = ui::Test::new();
    loop {
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        if let event::Event::Key(evt) = event::read().unwrap() {
            renderable.key(evt);
        }
        // match event::read().unwrap() {
        //     event::Event::Key(event::KeyEvent {
        //         code: event::KeyCode::Char('h'),
        //         modifiers: event::KeyModifiers::CONTROL,
        //     }) => {
        //         stdout
        //             .execute(CLEAR)
        //             .unwrap()
        //             .execute(style::Print("Hello World"))
        //             .unwrap();
        //     }
        //     event::Event::Key(event::KeyEvent {
        //         code: event::KeyCode::Char('t'),
        //         modifiers: event::KeyModifiers::ALT,
        //     }) => {
        //         stdout
        //             .execute(CLEAR)
        //             .unwrap()
        //             .execute(style::Print("crossterm is cool"))
        //             .unwrap();
        //     }
        //     event::Event::Key(event::KeyEvent {
        //         code: event::KeyCode::Char('z'),
        //         modifiers: event::KeyModifiers::NONE,
        //     }) => {
        //         stdout
        //             .execute(CLEAR)
        //             .unwrap()
        //             .execute(style::Print("100/100%!"))
        //             .unwrap();
        //     }
        //     event::Event::Key(event::KeyEvent {
        //         code: event::KeyCode::Char('q'),
        //         modifiers: event::KeyModifiers::CONTROL,
        //     }) => break,
        //     _ => (),
        // }
    }
}

fn main() {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();
    print_welcome(&mut stdout);
    detect_keypresses(&mut stdout);
    terminal::disable_raw_mode().unwrap();
}
