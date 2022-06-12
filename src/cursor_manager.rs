// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::{cursor, execute};
use std::io::stdout;

#[derive(Default)]
pub struct Cursor(pub u16, pub u16);

impl Cursor {
    pub fn move_left(&mut self) {
        if self.0 != 0 {
            self.0 -= 1;
            self.set();
        }
    }

    pub fn move_right(&mut self) {
        if self.0 != u16::MAX {
            self.0 += 1;
            self.set();
        }
    }

    pub fn move_top(&mut self) {
        if self.1 != 0 {
            self.1 -= 1;
            self.set();
        }
    }

    pub fn move_bottom(&mut self) {
        if self.1 != u16::MAX {
            self.1 += 1;
            self.set();
        }
    }

    pub fn reset(&mut self) {
        self.0 = 0;
        self.1 = 0;
        self.set();
    }

    pub fn set(&self) {
        execute!(stdout(), cursor::MoveTo(self.0, self.1));
    }
}
