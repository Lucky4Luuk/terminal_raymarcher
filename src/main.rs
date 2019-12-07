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

pub mod engine;
pub mod rendering;

#[derive(Debug)]
pub enum Event {
    QuitGame,
}

fn next_event(reader: &mut AsyncReader) -> Option<Event> {
    while let Some(event) = reader.next() {
        match event {
            //Example code commented out
            // InputEvent::Keyboard(key) => {
            //     if let Ok(new_direction) = Direction::try_from(key) {
            //         if snake_direction.can_change_to(new_direction) {
            //             return Some(Event::UpdateSnakeDirection(new_direction));
            //         }
            //     }
            // }
            InputEvent::Keyboard(KeyEvent::Esc) => return Some(Event::QuitGame),
            _ => {}
        }
    }
    None
}

fn main() -> Result<()> {
    let term_size: (u16, u16) = terminal::size()?;

    let mut stdin = input().read_async();

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Output(format!("========DEBUG========\nTerm size: [{}; {}]\n=====================\n", term_size.0, term_size.1)))?
        .execute(ResetColor)?
        .execute(cursor::DisableBlinking)?;

    loop {
        match next_event(&mut stdin) {
            Some(Event::QuitGame) => break,
            _ => {}
        };
    }

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Output("Thanks for playing!"))?
        .execute(cursor::EnableBlinking)?
        .execute(ResetColor)?;

    Ok(())
}
