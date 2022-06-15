// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[macro_export]
macro_rules! clear {
    () => {
        execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .ok();
    };
}

#[macro_export]
macro_rules! cursor_pos {
    ($x:expr, $y:expr) => {
        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo($x, $y)).ok();
    };
}
