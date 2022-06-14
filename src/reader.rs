// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::mem::take;
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

    pub fn delete_character(&mut self, cursor: &Cursor) {
        let line = cursor.main.y;
        let column = cursor.main.x;

        if line >= self.lines.len() {
            return;
        }

        let mut selected = take(&mut self.lines[line]);

        if selected.is_empty() {
            self.lines.remove(line);
            return;
        } else if column < selected.len() {
            selected.remove(column);
        }

        self.lines[line] = selected;
    }

    pub fn add_character(&mut self, cursor: &Cursor, character: char) {
        let line = cursor.main.y;
        let column = cursor.main.x;

        if line >= self.lines.len() {
            for _ in self.lines.len()..=line {
                self.lines.push(String::new());
            }
        }

        let mut selected = take(&mut self.lines[line]);

        if column >= selected.len() {
            for _ in selected.len()..=column {
                selected.push(' ');
            }
        }
        selected.insert(column, character);
        self.lines[line] = selected;
    }

    pub fn add_line(&mut self, cursor: &Cursor) {
        let line = cursor.main.y;

        if line >= self.lines.len() {
            for _ in self.lines.len()..=line {
                self.lines.push(String::new());
            }
        }

        self.lines.insert(line, String::new());
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
