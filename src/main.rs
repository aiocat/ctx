use crossterm::terminal;

mod cursor_manager;
mod input;
mod manager;

fn main() {
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");

    let manager = manager::Manager::new();
    manager.clear();
    manager.watch();
}
