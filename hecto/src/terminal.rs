use std::io::{self, stdout, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::editor::Position;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    /// Creates default terminal
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] if [`termion::terminal_size()`] or
    /// [`termion::raw::IntoRawMode`] failed.
    pub fn default() -> Result<Self> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    #[must_use]
    pub const fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x.try_into().unwrap_or(u16::MAX);
        let y = y.try_into().unwrap_or(u16::MAX);
        print!("{}", termion::cursor::Goto(x, y));
    }

    /// Wraper for [`std::io::stdout`] flush
    /// # Errors
    /// Same as stdout().flush()
    pub fn flush() -> Result<()> {
        io::stdout().flush()
    }

    ///
    /// # Errors
    /// Cannot get key.
    pub fn read_key() -> Result<Key> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub(crate) fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}
