use crossterm::terminal;

mod cursor_manager;
mod input;
mod manager;
mod reader;

fn main() {
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");

    let mut manager = manager::Manager::new();
    manager.cursor.reset();
    manager.clear();
    manager.watch();
}
