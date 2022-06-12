// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::event;
use crossterm::event::{Event, KeyEvent};

pub fn watch_key<F>(mut callback: F)
where
    F: FnMut(KeyEvent),
{
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            callback(event);
        };
    }
}
