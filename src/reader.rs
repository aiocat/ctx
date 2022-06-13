// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

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

    pub fn print_lines(&mut self, start: usize, max: u16) {
        let mut count: u16 = 0;

        for line in &self.lines[start..] {
            count += 1;
            if count == max {
                break;
            }

            println!("{}", line);
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
