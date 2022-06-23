<!--
 Copyright (c) 2022 aiocat
 
 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

# CTX
CTX, is a simple terminal-based text editor written in Rust. CTX does not using a TUI library like `tui-rs`, instead it is using `crossterm` to send terminal commands cross-platform. Didn't tested on Linux/MacOS, but works on Windows 10.

## Screenshot
![](/assets/ctx.png)

## Note
This editor is supports UTF-8, and for this may buffer-manupilation codes looks verbose. (because of re-inventing the wheel)

## Usage
- To open a file run `ctx ./file.txt`
- To save the file, press `CTRL + S`
- To delete a line, press `CTRL + D`
- To undo what did you do, press `CTRL + U` (only one command can be reversed)
- To split-up, press `CTRL + UP ARROW`
- To split-down, press `CTRL + DOWN ARROW`
- To enter a new line, press `ENTER`
- To remove a character, press `BACKSPACE`
- To append a character, just press the key that you want to add
- Also if you want to exit, just press `ESC`
- You can move your cursor with `ARROW` keys, has a flexible movement system!

## Why?
Well, i did this project to test myself. But if you want to make a text-editor in Rust and have no idea, just look at the source code! So it is can be used in educational purposes.

## License
CTX is licensed under the MIT license.