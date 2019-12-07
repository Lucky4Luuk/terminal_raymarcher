use std::io::{stdout, Write};

use crossterm::{
    execute,
    terminal,
    cursor,
    input::{input, AsyncReader, InputEvent, KeyEvent},
    ExecutableCommand,
    style::{Attribute, Color, SetForegroundColor, SetBackgroundColor, ResetColor},
    Output,
    Result
};

pub struct Screen {
    pub buffer: Vec<Vec<char>>, //layout is y; x
    pub size: (u16, u16),
}

impl Screen {
    pub fn new(term_size: (u16, u16)) -> Screen {
        let mut buffer = Vec::new();

        for y in 0.. term_size.1 {
            // println!("y: {}", y as usize);
            buffer.push(Vec::new());
            for x in 0.. term_size.0 {
                buffer[y as usize].push(' ');
            }
        }

        Screen {
            buffer: buffer,
            size: term_size,
        }
    }

    pub fn set(&mut self, pos: (u16, u16), value: char) {
        self.buffer[pos.1 as usize][pos.0 as usize] = value;
    }

    pub fn render(&self) {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        for y in 0.. self.size.1 {
            let mut s: String = self.buffer[y as usize].iter().collect();
            if y < self.size.1 - 1 {
                s.push('\n');
            }
            stdout()
                .execute(Output(s)).unwrap();
        }
    }
}
