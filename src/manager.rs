// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::{execute, terminal};
use r256::Styles;

use crate::buffer_manager::Buffer;
use crate::cursor_manager::{Cursor, MainCursor};
use crate::input::watch_key;

#[derive(Default)]
pub struct Size(pub u16, pub u16);

pub struct Manager {
    pub size: Size,
    pub cursor: Cursor,
    pub buffer: Buffer,
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
            buffer: Buffer::default(),
        }
    }

    fn handle_arrows(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => self.cursor.move_left(self.size.0),
            KeyCode::Right => self.cursor.move_right(self.size.0),
            KeyCode::Up => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.buffer.split_to_up(&self.cursor);
                } else {
                    self.cursor.move_top(self.size.1);
                }
            }
            KeyCode::Down => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.buffer.split_to_down(&self.cursor);
                } else {
                    self.cursor.move_bottom(self.size.1);
                }
            }
            _ => {}
        };
    }

    pub fn watch(&mut self) {
        self.read_file();
        self.handle_buffer();
        self.set_title();

        watch_key(move |key: KeyEvent| {
            match key.code {
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    self.handle_arrows(key);
                    self.handle_buffer();
                }
                KeyCode::Char('r') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        self.resize();
                        self.handle_buffer();
                    } else if let KeyCode::Char(character) = key.code {
                        self.type_char(character);
                    }
                }
                KeyCode::Char('s') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        self.buffer.save_buffer();
                    } else if let KeyCode::Char(character) = key.code {
                        self.type_char(character);
                    }
                }
                KeyCode::Char('d') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        self.buffer.remove_line(&self.cursor);
                        self.handle_buffer();
                    } else if let KeyCode::Char(character) = key.code {
                        self.type_char(character);
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
                    self.type_char(given);
                }
                _ => {}
            };
            self.set_title();
        });
    }

    fn type_char(&mut self, character: char) {
        self.buffer.add_character(&self.cursor, character);
        self.handle_arrows(KeyEvent::from(KeyCode::Right));
        self.handle_buffer();
    }

    pub fn clear(&mut self) {
        crate::clear!();
    }

    fn resize(&mut self) {
        let win_size = terminal::size().map(|(x, y)| (x as u16, y as u16)).unwrap();
        self.size = Size(win_size.0, win_size.1);
        self.cursor.reset();
        self.handle_buffer();
    }

    fn set_title(&mut self) {
        crate::cursor_pos!(0, 0);
        println!("{:0fill$}", '\r', fill = self.size.0 as usize - 1);
        crate::cursor_pos!(0, 0);

        r256::print(
            &vec![Styles::FgColor256(13), Styles::Bold, Styles::Italic],
            &format!(
                "[CTX] LINE {}, COLUMN {}",
                self.cursor.main.y + 1,
                self.cursor.main.x
            ),
        );

        self.cursor.set();
    }

    fn read_file(&mut self) {
        self.buffer.read_from_file("./deneme.txt");
    }

    fn nearest_cursors(&mut self) -> (usize, usize) {
        let calculated_size_x = self.size.0 as usize - 3;
        let calculated_size_y = self.size.1 as usize - 2;

        (
            ((self.cursor.main.x / calculated_size_x) * calculated_size_x),
            ((self.cursor.main.y / calculated_size_y) * calculated_size_y),
        )
    }

    fn handle_buffer(&mut self) {
        self.clear();
        crate::cursor_pos!(0, 0);

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
