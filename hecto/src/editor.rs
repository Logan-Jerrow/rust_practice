use crate::{Document, Row, Terminal};
use std::{
    env,
    time::{Duration, Instant},
};
use termion::{color, event::Key};

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
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

struct StatusMessage {
    text: String,
    time: Instant,
}

impl From<String> for StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}

/// Representing a text editor.
pub struct Editor {
    quit_flag: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
}

impl Default for Editor {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-Q = quit");
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(file_name).unwrap_or_else(|_| {
                initial_status = format!("ERROR: Could not open file: {file_name}");
                Document::default()
            })
        } else {
            Document::default()
        };

        Self {
            quit_flag: false,
            terminal: Terminal::default().expect("Failed to initialize terminal."),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            status_message: StatusMessage::from(initial_status),
        }
    }
}

impl Editor {
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

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit_flag = true,
            is_movement_key!() => self.move_cursor(pressed_key),
            // _ if Self::is_movement_key(pressed_key) => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position {
            x: cursor_x,
            y: cursor_y,
        } = self.cursor_position;

        let width: usize = self.terminal.size().width.into();
        let height: usize = self.terminal.size().height.into();

        let mut offset = &mut self.offset;

        if cursor_y < offset.y {
            offset.y = cursor_y;
        } else if cursor_y >= offset.y.saturating_add(height) {
            offset.y = cursor_y.saturating_sub(height).saturating_add(1);
        }

        if cursor_x < offset.x {
            offset.x = cursor_x;
        } else if cursor_x >= offset.x.saturating_add(width) {
            offset.x = cursor_x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let terminal_height: usize = self.terminal.size().height.into();
        let Position {
            x: mut cursor_x,
            y: mut cursor_y,
        } = self.cursor_position;

        let doc_height: usize = self.document.len();
        let mut doc_width: usize = self.document.row(cursor_y).map_or(0, Row::len);

        match key {
            Key::Up => cursor_y = cursor_y.saturating_sub(1),
            Key::Down => {
                if cursor_y < doc_height {
                    cursor_y = cursor_y.saturating_add(1);
                }
            }
            Key::Left => {
                if cursor_x > 0 {
                    cursor_x -= 1;
                } else if cursor_y > 0 {
                    cursor_y -= 1;
                    cursor_x = self.document.row(cursor_y).map_or(0, Row::len);
                }
            }
            Key::Right => {
                if cursor_x < doc_width {
                    cursor_x += 1;
                } else if cursor_y < doc_height {
                    cursor_y += 1;
                    cursor_x = 0;
                }
            }
            Key::PageUp => {
                cursor_y = if cursor_y > terminal_height {
                    cursor_y - (terminal_height / 2)
                } else {
                    0
                }
            }
            Key::PageDown => {
                cursor_y = if cursor_y.saturating_add(terminal_height) < doc_height {
                    cursor_y + (terminal_height / 2)
                } else {
                    doc_height
                }
            }
            Key::Home => cursor_x = 0,
            Key::End => cursor_x = doc_width,
            _ => unreachable!(),
        }
        doc_width = self.document.row(cursor_y).map_or(0, Row::len);
        if cursor_x > doc_width {
            cursor_x = doc_width;
        }
        self.cursor_position = Position {
            x: cursor_x,
            y: cursor_y,
        };
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.quit_flag {
            Terminal::clear_screen();
            println!("Goodbye\r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn draw_row(&self, row: &Row) {
        let width: usize = self.terminal.size().width.into();
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{row}\r");
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        // -1 to keep from scolling
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            self.document
                .row(usize::from(terminal_row) + self.offset.y)
                .map_or_else(
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

    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width.into();
        let file_name = self.document.file_name.as_ref().map_or_else(
            || "[No Name]".to_string(),
            |name| {
                let mut name = name.clone();
                name.truncate(20);
                name
            },
        );

        status = format!("{file_name} - {} lines", self.document.len());

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len(),
        );

        let length = status.len() + line_indicator.len();
        if width > length {
            status.push_str(&" ".repeat(width - length));
        }
        status = format!("{status}{line_indicator}");
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{status}\r");
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width.into());
            print!("{text}");
        }
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    let e = e;
    panic!("{e}");
}
