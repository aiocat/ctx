// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::event::{KeyCode, KeyEvent};
use crossterm::terminal::ClearType;
use crossterm::{execute, terminal};
use std::cell::Cell;
use std::io::stdout;
use std::process;

use crate::cursor_manager::Cursor;
use crate::input::watch_key;

#[derive(Default)]
pub struct Size(pub u16, pub u16);

pub struct Manager {
    size: Cell<Size>,
    cursor: Cell<Cursor>,
}

impl Manager {
    pub fn new() -> Self {
        let win_size = terminal::size().map(|(x, y)| (x as u16, y as u16)).unwrap();
        Self {
            size: Cell::new(Size(win_size.0, win_size.1)),
            cursor: Cell::new(Cursor(0, 0)),
        }
    }

    fn handle_arrows(&self, key: KeyEvent) {
        let size = self.size.take();
        let mut cursor = self.cursor.take();

        match key.code {
            KeyCode::Left => cursor.move_left(),
            KeyCode::Right => cursor.move_right(size.0),
            KeyCode::Up => cursor.move_top(),
            KeyCode::Down => cursor.move_bottom(size.1),
            _ => todo!(),
        };

        self.cursor.set(cursor);
        self.size.set(size);
    }

    fn handle_exit(&self) {
        drop(self.size.take());
        drop(self.cursor.take());

        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        execute!(stdout(), terminal::Clear(ClearType::All)).ok();

        process::exit(0);
    }

    pub fn watch(&self) {
        watch_key(move |key: KeyEvent| match key.code {
            KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                self.handle_arrows(key);
            }
            KeyCode::Esc => {
                self.handle_exit();
            }
            _ => todo!(),
        });
    }

    pub fn clear(&self) {
        execute!(stdout(), terminal::Clear(ClearType::All)).ok();

        let mut cursor = self.cursor.take();
        cursor.reset();
        self.cursor.set(cursor);
    }
}
