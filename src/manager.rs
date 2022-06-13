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
            KeyCode::Left => self.cursor.move_left(),
            KeyCode::Right => self.cursor.move_right(self.size.0),
            KeyCode::Up => self.cursor.move_top(),
            KeyCode::Down => self.cursor.move_bottom(self.size.1),
            _ => {}
        };
    }

    pub fn watch(&mut self) {
        self.read_file();
        self.set_title();
        self.cursor.reset();
        self.handle_buffer();

        watch_key(move |key: KeyEvent| {
            match key.code {
                KeyCode::Left | KeyCode::Right => {
                    self.handle_arrows(key);
                    self.set_title();
                    self.handle_buffer_left_right();
                }
                KeyCode::Up | KeyCode::Down => {
                    self.handle_arrows(key);
                    self.set_title();
                    self.handle_buffer_up_down();
                }
                KeyCode::Char('r') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        self.resize();
                    }
                }
                KeyCode::Backspace => {
                    self.buffer.delete_character(&self.cursor);
                    self.cursor.move_left();
                    self.resize();
                }
                _ => {}
            };
        });
    }

    pub fn clear(&mut self) {
        execute!(stdout(), terminal::Clear(ClearType::All)).ok();
    }

    fn resize(&mut self) {
        let win_size = terminal::size().map(|(x, y)| (x as u16, y as u16)).unwrap();
        self.size = Size(win_size.0, win_size.1);
        self.handle_buffer();
    }

    fn set_title(&mut self) {
        execute!(
            stdout(),
            terminal::SetTitle(format!(
                "Line {}, Column {}",
                self.cursor.main.y + 1,
                self.cursor.main.x
            ))
        )
        .ok();
    }

    fn read_file(&mut self) {
        self.buffer.read_from_file("./deneme.txt");
    }

    fn handle_buffer_up_down(&mut self) {
        self.clear();
        execute!(stdout(), cursor::MoveTo(0, 0)).ok();
        self.buffer.print_lines(&self.cursor, &self.size);
        self.cursor.reset_only_y();
    }

    fn handle_buffer_left_right(&mut self) {
        self.clear();
        execute!(stdout(), cursor::MoveTo(0, 0)).ok();
        self.buffer.print_lines(&self.cursor, &self.size);
        self.cursor.reset_only_x();
    }

    fn handle_buffer(&mut self) {
        self.clear();
        execute!(stdout(), cursor::MoveTo(0, 0)).ok();
        self.buffer.print_lines(&self.cursor, &self.size);
        self.cursor.reset_only();
    }
}
