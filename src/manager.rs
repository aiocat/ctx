// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::event::{KeyCode, KeyEvent};
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal};
use std::io::stdout;

use crate::cursor_manager::{Cursor, MainCursor};
use crate::input::watch_key;
use crate::reader::Reader;

#[derive(Default)]
pub struct Size(pub u16, pub u16);

pub struct Manager {
    pub size: Size,
    pub cursor: Cursor,
    pub reader: Reader,
}

impl Manager {
    pub fn new() -> Self {
        let win_size = terminal::size().map(|(x, y)| (x as u16, y as u16)).unwrap();

        Self {
            size: Size(win_size.0, win_size.1),
            cursor: Cursor {
                x: 0,
                y: 0,
                main: MainCursor { x: 0, y: 0 },
            },
            reader: Reader::default(),
        }
    }

    fn handle_arrows(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => self.cursor.move_left(),
            KeyCode::Right => self.cursor.move_right(self.size.0),
            KeyCode::Up => {
                self.cursor.move_top();
                self.resize();
            }
            KeyCode::Down => {
                self.cursor.move_bottom(self.size.1);
                self.resize();
            }
            _ => {}
        };
    }

    pub fn watch(&mut self) {
        self.read_file();
        self.resize();
        self.cursor.reset();

        watch_key(move |key: KeyEvent| {
            self.resize();
            match key.code {
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    self.handle_arrows(key)
                }
                _ => {}
            };
        });
    }

    pub fn clear(&mut self) {
        execute!(stdout(), terminal::Clear(ClearType::All)).ok();
    }

    fn read_file(&mut self) {
        self.reader.read_from_file("./deneme.txt");
    }

    fn resize(&mut self) {
        let win_size = terminal::size().map(|(x, y)| (x as u16, y as u16)).unwrap();
        self.size = Size(win_size.0, win_size.1);
        self.handle_buffer();
    }

    fn handle_buffer(&mut self) {
        if self.cursor.y % (self.size.1 - 1) == 0 {
            self.clear();
            execute!(stdout(), cursor::MoveTo(0, 0)).ok();
            self.reader.print_lines(self.cursor.main.y, self.size.1);
            self.cursor.reset_only_y();
        }
    }
}
