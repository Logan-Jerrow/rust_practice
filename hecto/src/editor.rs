use crate::{Document, Row, Terminal};
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! is_movement_key {
    () => {
        Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home
    };
}

#[derive(Debug, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Representing a text editor.
pub struct Editor {
    quit_flag: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}

impl Editor {
    /// Provides a default configuration of [`Editor`]
    pub fn default() -> Self {
        Self {
            quit_flag: false,
            terminal: Terminal::default().expect("Failed to initialize terminal."),
            cursor_position: Position::default(),
            document: Document::open(),
            // document: Document::default(),
        }
    }

    /// Runs the Editor.
    ///
    /// Hooks up to stdin outputs to stdout raw.
    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                die(e);
            }
            if self.quit_flag {
                break;
            }
            if let Err(e) = self.process_keypress() {
                die(e);
            }
        }
    }

    const fn is_movement_key(key: Key) -> bool {
        matches!(
            key,
            Key::Up
                | Key::Down
                | Key::Left
                | Key::Right
                | Key::PageUp
                | Key::PageDown
                | Key::End
                | Key::Home
        )
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit_flag = true,
            is_movement_key!() => self.move_cursor(pressed_key),
            // _ if Self::is_movement_key(pressed_key) => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;

        let size = self.terminal.size();
        let height: usize = size.height.saturating_sub(1).into();
        let width: usize = size.width.saturating_sub(1).into();

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => unreachable!(),
        }
        self.cursor_position = Position { x, y };
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.quit_flag {
            Terminal::clear_screen();
            println!("Goodbye\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width.into();
        let row = row.render(start, end);
        println!("{row}\r");
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        // -1 to keep from scolling
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            self.document.row(terminal_row.into()).map_or_else(
                || {
                    if self.document.is_empty() && terminal_row == height / 3 {
                        self.draw_welcome_message();
                    } else {
                        println!("~\r");
                    }
                },
                |row| {
                    self.draw_row(row);
                },
            );
        }
    }

    fn draw_welcome_message(&self) {
        let width: usize = self.terminal.size().width.into();
        let mut welcome_msg = format!("Hecto editor -- version {VERSION}");

        let len = welcome_msg.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1)); // -1 for tilda (~)

        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);

        println!("{welcome_msg}\r");
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    let e = e;
    panic!("{e}");
}
