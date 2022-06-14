// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
    pub buffer: Reader,
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
            buffer: Reader::default(),
        }
    }

    fn handle_arrows(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => self.cursor.move_left(self.size.0),
            KeyCode::Right => self.cursor.move_right(self.size.0),
            KeyCode::Up => self.cursor.move_top(self.size.1),
            KeyCode::Down => self.cursor.move_bottom(self.size.1),
            _ => {}
        };
    }

    pub fn watch(&mut self) {
        self.read_file();
        self.set_title();
        self.handle_buffer();

        watch_key(move |key: KeyEvent| {
            match key.code {
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    self.handle_arrows(key);
                    self.handle_buffer();
                }
                KeyCode::Char('r') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        self.resize();
                    } else if let KeyCode::Char(character) = key.code {
                        self.buffer.add_character(&self.cursor, character);
                        self.handle_arrows(KeyEvent::from(KeyCode::Right));
                        self.handle_buffer();
                    }
                }
                KeyCode::Backspace => {
                    self.handle_arrows(KeyEvent::from(KeyCode::Left));
                    self.buffer.delete_character(&self.cursor);
                    self.handle_buffer();
                }
                KeyCode::Enter => {
                    self.buffer.add_line(&self.cursor);
                    self.handle_arrows(KeyEvent::from(KeyCode::Down));
                    self.handle_buffer();
                }
                KeyCode::Char(given) => {
                    self.buffer.add_character(&self.cursor, given);
                    self.handle_arrows(KeyEvent::from(KeyCode::Right));
                    self.handle_buffer();
                }
                _ => {}
            };
            self.set_title();
        });
    }

    pub fn clear(&mut self) {
        execute!(stdout(), terminal::Clear(ClearType::All)).ok();
    }

    fn resize(&mut self) {
        let win_size = terminal::size().map(|(x, y)| (x as u16, y as u16)).unwrap();
        self.size = Size(win_size.0, win_size.1);
        self.cursor.reset();
        self.handle_buffer();
    }

    fn set_title(&mut self) {
        execute!(
            stdout(),
            terminal::SetTitle(format!(
                "LINE {} COLUMN {}",
                self.cursor.main.y, self.cursor.main.x
            ))
        )
        .ok();
    }

    fn read_file(&mut self) {
        self.buffer.read_from_file("./deneme.txt");
    }

    fn nearest_cursors(&mut self) -> (usize, usize) {
        (
            ((self.cursor.main.x as f64 / (self.size.0 - 1) as f64).floor() as usize
                * self.size.0 as usize),
            ((self.cursor.main.y as f64 / (self.size.1 - 1) as f64).floor() as usize
                * self.size.1 as usize),
        )
    }

    fn handle_buffer(&mut self) {
        self.clear();

        execute!(stdout(), cursor::MoveTo(0, 0)).ok();

        let (old_cursor_x, old_cursor_y) = (self.cursor.main.x, self.cursor.main.y);
        let nearest = self.nearest_cursors();

        self.cursor.main.x = nearest.0;
        self.cursor.main.y = nearest.1;

        self.buffer.print_lines(&self.cursor, &self.size);

        self.cursor.main.x = old_cursor_x;
        self.cursor.main.y = old_cursor_y;

        self.cursor.set();
    }
}
