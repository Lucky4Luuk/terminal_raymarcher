use std::io::{stdout, Write};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    pub buffer: Vec<Vec<(char, Color)>>, //layout is y; x
    pub size: (u16, u16),
}

impl Screen {
    pub fn new(term_size: (u16, u16)) -> Screen {
        let mut buffer = Vec::new();

        for y in 0.. term_size.1 {
            // println!("y: {}", y as usize);
            buffer.push(Vec::new());
            for _x in 0.. term_size.0 {
                buffer[y as usize].push((' ', Color::Reset));
            }
        }

        Screen {
            buffer: buffer,
            size: term_size,
        }
    }

    pub fn set(&mut self, pos: (u16, u16), value: (char, Color)) {
        self.buffer[pos.1 as usize][pos.0 as usize] = value;
    }

    //TODO: Error handling lol
    pub fn render(&self) {
        // stdout()
        //     .execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout()
            .execute(cursor::MoveTo(0,0)).unwrap();
        for y in 0.. self.size.1 {
            //Following commented line allows for very fast grayscale output, if the type if char and not (char, Color)
            // let mut s: String = self.buffer[y as usize].iter().collect();
            let mut s = String::new();
            for x in 0.. self.size.0 {
                let (value, color) = self.buffer[y as usize][x as usize];
                s.push_str(&*format!("{}", SetForegroundColor(color)));
                s.push(value);
            }
            if y < self.size.1 - 1 {
                s.push('\n');
            }
            stdout()
                .execute(Output(s)).unwrap();
        }
    }
}
