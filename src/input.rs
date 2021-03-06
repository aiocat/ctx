// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{execute, terminal};

pub fn watch_key<F>(mut callback: F)
where
    F: FnMut(KeyEvent),
{
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            if event.code == KeyCode::Esc {
                terminal::disable_raw_mode().expect("Unable to disable raw mode");
                crate::cursor_pos!(0, 0);
                crate::clear!();

                break;
            }

            callback(event);
        };
    }
}
