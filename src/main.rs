use crossterm::terminal;

mod buffer_manager;
mod cursor_manager;
mod input;
mod macros;
mod manager;

fn main() {
    r256::init();
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");

    let mut manager = manager::Manager::new();
    manager.cursor.reset();
    manager.clear();
    manager.watch();
}
