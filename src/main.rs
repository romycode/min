use std::io::stdout;

use crossterm::event;

use crate::buffer::Buffer;
use crate::term::Term;

mod buffer;
mod term;

fn main() {
    let stdout = stdout();
    let mut term = Term::new(&stdout);

    term.enable_raw();
    term.clear();
    term.flush();

    let mut buffer = Buffer::new();

    loop {
        term.clear();
        term.move_cursor(0, 0);
        term.print(&buffer.content());
        term.flush();

        if let Ok(event) = term.event() {
            match event {
                event::Event::Key(key) => match key {
                    event::KeyEvent {
                        code: event::KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::ALT,
                        ..
                    } => break,
                    event::KeyEvent {
                        code: event::KeyCode::Enter,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        buffer.insert('\n');
                    },
                    event::KeyEvent {
                        code: event::KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        buffer.remove();
                    },
                    event::KeyEvent {
                        code: event::KeyCode::Char(c),
                        modifiers: event::KeyModifiers::NONE | event::KeyModifiers::SHIFT,
                        ..
                    } => {
                        buffer.insert(c);
                    }
                    _ => {}
                }
                event::Event::FocusGained => {}
                event::Event::FocusLost => {}
                event::Event::Mouse(_) => {}
                event::Event::Paste(_) => {}
                event::Event::Resize(_, _) => {}
            }
        }
    }
}