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
    pub buffer: Vec<Vec<(char, Color, Color)>>, //layout is y; x
    pub size: (u16, u16),
}

impl Screen {
    pub fn new(term_size: (u16, u16)) -> Screen {
        let mut buffer = Vec::new();

        for y in 0.. term_size.1 {
            // println!("y: {}", y as usize);
            buffer.push(Vec::new());
            for _x in 0.. term_size.0 {
                buffer[y as usize].push((' ', Color::Reset, Color::Reset));
            }
        }

        Screen {
            buffer: buffer,
            size: term_size,
        }
    }

    //TODO: Remove term_size parameter and replace it with self.size
    pub fn flush(&mut self, term_size: (u16, u16), colors: (Color, Color)) {
        self.buffer = Vec::new();

        for y in 0.. term_size.1 {
            // println!("y: {}", y as usize);
            self.buffer.push(Vec::new());
            for _x in 0.. term_size.0 {
                self.buffer[y as usize].push((' ', colors.0, colors.1));
            }
        }
    }

    pub fn set(&mut self, pos: (u16, u16), value: (char, Color)) {
        self.buffer[pos.1 as usize][pos.0 as usize].0 = value.0;
        self.buffer[pos.1 as usize][pos.0 as usize].1 = value.1;
    }

    pub fn set_bg(&mut self, pos: (u16, u16), bg_col: Color) {
        self.buffer[pos.1 as usize][pos.0 as usize].2 = bg_col;
    }

    //TODO: Error handling lol
    //NOTE: I made it ever so slightly faster on windows, by making it output the entire screen
    //      at once, however, on linux, this completely breaks everything
    pub fn render(&self) {
        // stdout()
        //     .execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout()
            .execute(cursor::MoveTo(0,0)).unwrap();
        // let mut s = String::new();
        for y in 0.. self.size.1 {
            //Following commented line allows for very fast grayscale output, if the type if char and not (char, Color)
            // let mut s: String = self.buffer[y as usize].iter().collect();
            let mut s = String::new();
            for x in 0.. self.size.0 {
                let (value, color, bg_col) = self.buffer[y as usize][x as usize];
                s.push_str(&*format!("{}", SetBackgroundColor(bg_col)));
                s.push_str(&*format!("{}", SetForegroundColor(color)));
                s.push(value);
            }
            // if y < self.size.1 - 1 {
            //     s.push('\n');
            // }
            stdout()
                .execute(Output(s)).unwrap();
        }
        // stdout()
        //     .execute(Output(s)).unwrap();
    }
}
