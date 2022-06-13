// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::{cursor, execute};
use std::io::stdout;

#[derive(Default, Debug)]
pub struct MainCursor {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Debug)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub main: MainCursor,
}

impl Cursor {
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
            self.set();
        }

        if self.main.x > 0 {
            self.main.x -= 1;
        }
    }

    pub fn move_right(&mut self, max: u16) {
        if self.x < max {
            self.x += 1;
            self.set();
        }

        if self.main.x < usize::MAX {
            self.main.x += 1;
        }
    }

    pub fn move_top(&mut self) {
        if self.y > 0 {
            self.y -= 1;
            self.set();
        }

        if self.main.y > 0 {
            self.main.y -= 1;
        }
    }

    pub fn move_bottom(&mut self, max: u16) {
        if self.y < max {
            self.y += 1;
            self.set();
        }

        if self.main.y < usize::MAX {
            self.main.y += 1;
        }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.main.x = 0;
        self.main.y = 0;
        self.set();
    }

    pub fn reset_only(&mut self) {
        self.x = 0;
        self.y = 0;
        self.set();
    }

    pub fn reset_only_x(&mut self) {
        self.x = 0;
        self.set();
    }

    pub fn reset_only_y(&mut self) {
        self.y = 0;
        self.set();
    }

    pub fn set(&self) {
        execute!(stdout(), cursor::MoveTo(self.x, self.y)).ok();
    }
}
