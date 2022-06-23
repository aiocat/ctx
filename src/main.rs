use std::env;

use crossterm::terminal;

mod buffer_manager;
mod cursor_manager;
mod input;
mod macros;
mod manager;

fn main() {
    let file_name = get_file_name();

    r256::init();
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");

    let mut manager = manager::Manager::new(file_name);
    manager.cursor.reset();
    manager.clear();
    manager.watch();
}

fn get_file_name() -> String {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("Wrong usage, please run like: ctx ./file.txt");
    }

    let file_name = args[1].clone();
    file_name
}
