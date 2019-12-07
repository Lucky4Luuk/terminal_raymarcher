use std::io::{stdout, Write};
use std::{thread, time};

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
use engine::{
    distance_field::*,
    scene::Scene,
};

pub mod rendering;
use rendering::{
    screen::Screen,
    raymarching as rm,
};

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

    let mut screen = Screen::new(term_size);

    // screen.buffer[0][0] = 'x';
    // screen.buffer[(term_size.1 - 1) as usize][(term_size.0 - 1) as usize] = 'a';

    screen.set((1, 1), 'x');
    screen.set((term_size.0 - 1, term_size.1 - 1), 'a');

    let mut scene = Scene::new();
    let sphere = SDF::new_sphere([0.0, 0.0, 5.0], 0.5);
    scene.distance_fields.push(sphere);

    loop {
        match next_event(&mut stdin) {
            Some(Event::QuitGame) => break,
            _ => {}
        };

        screen.render();
        // break;

        thread::sleep(time::Duration::from_millis(200));
    }

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Output("Thanks for using!"))?
        .execute(cursor::EnableBlinking)?
        .execute(ResetColor)?;

    Ok(())
}
