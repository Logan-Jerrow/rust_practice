//! Follow the hecto [`guide`] on how to build your own text editor in rust.
//!
//! [`guide`]: "https://www.philippflenker.com/hecto/"

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use editor::Editor;

mod editor;
mod terminal;

pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
