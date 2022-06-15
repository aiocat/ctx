// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fs::File;
use std::io::stdout;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::mem::take;
use std::path::Path;

use crate::cursor_manager::Cursor;
use crate::manager::Size;

#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn read_from_file<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        self.lines = read_file_lines(path);
    }

    pub fn print_lines(&mut self, cursor: &Cursor, size: &Size) {
        println!();

        let mut stdout = stdout();
        let mut count: u16 = 1;

        if self.lines.len() <= cursor.main.y {
            println!();
            return;
        }

        for line in &self.lines[cursor.main.y..] {
            count += 1;
            if count == size.1 {
                break;
            }

            print!("~ ");
            let mut char_count: u16 = 2;
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

                stdout.write_all(&[*character]).ok();
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
            let mut new_selected = String::new();

            for (index, schar) in selected.chars().enumerate() {
                if index == column {
                    continue;
                }

                new_selected.push(schar);
            }

            selected = new_selected;
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

        let mut new_string = String::new();

        for (index, schar) in selected.chars().enumerate() {
            if index == column {
                new_string.push(character);
            }
            new_string.push(schar);
        }

        self.lines[line] = new_string;
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

    pub fn save_buffer(&mut self) {
        let mut new_buffer = String::new();

        for buf in &self.lines {
            new_buffer.push_str(buf);
            new_buffer.push('\n');
        }

        new_buffer.pop();

        std::fs::write("./deneme.txt", new_buffer).ok();
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
