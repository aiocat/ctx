use crossterm::terminal;

mod input;
mod cursor_manager;
mod manager;

fn main() {
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");

    let manager = manager::Manager::new();
    manager.clear();
    manager.watch();
}
