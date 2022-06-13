// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::cursor_manager::Cursor;
use crate::manager::Size;

#[derive(Default)]
pub struct Reader {
    lines: Vec<String>,
}

impl Reader {
    pub fn read_from_file<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        self.lines = read_file_lines(path);
    }

    pub fn print_lines(&mut self, cursor: &Cursor, size: &Size) {
        let mut count: u16 = 0;

        if self.lines.len() < cursor.main.y {
            println!();
            return;
        }

        for line in &self.lines[cursor.main.y..] {
            count += 1;
            if count == size.1 {
                break;
            }

            let mut char_count: u16 = 0;
            let line_bytes = line.as_bytes();

            if line_bytes.len() < cursor.main.x {
                println!();
                continue;
            }

            for character in &line.as_bytes()[cursor.main.x..] {
                char_count += 1;
                if char_count == size.0 {
                    break;
                }

                print!("{}", *character as char);
            }

            println!();
        }
    }
}

fn read_file_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Failed to open the file");
    let content = BufReader::new(file);

    let lines: Vec<String> = content
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();

    lines
}
