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

extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

#[derive(Debug)]
pub enum Event {
    HandleInput(KeyEvent),
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
            InputEvent::Keyboard(key) => return Some(Event::HandleInput(key)),
            _ => {}
        }
    }
    None
}

fn handle_input(scene: &mut Scene, key: KeyEvent) {
    if key == KeyEvent::Char('d') {
        scene.camera.yaw += 2.0;
    }
    if key == KeyEvent::Char('a') {
        scene.camera.yaw -= 2.0;
    }
}

fn main() -> Result<()> {
    let term_size: (u16, u16) = terminal::size()?;

    let _raw = crossterm::screen::RawScreen::into_raw_mode();
    let mut stdin = input().read_async();

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Output(format!("========DEBUG========\nTerm size: [{}; {}]\n=====================\n", term_size.0, term_size.1)))?
        .execute(ResetColor)?
        .execute(cursor::DisableBlinking)?;

    let mut screen = Screen::new(term_size);

    // screen.buffer[0][0] = 'x';
    // screen.buffer[(term_size.1 - 1) as usize][(term_size.0 - 1) as usize] = 'a';

    // screen.set((1, 1), 'x');
    // screen.set((term_size.0 - 1, term_size.1 - 1), 'a');

    let mut scene = Scene::new();
    let sphere = SDF::new_sphere([0.0, 0.0, 5.0], 1.0, [255, 0, 0]);
    scene.distance_fields.push(sphere);
    let plane = SDF::new_plane(-1.0, [255, 255, 255]);
    scene.distance_fields.push(plane);
    // let cube = SDF::new_cube([0.0, 0.0, 5.0], [0.5, 0.5, 0.5]);
    // scene.distance_fields.push(cube);

    //TODO: measure deltatime
    'main: loop {
        match next_event(&mut stdin) {
            Some(Event::HandleInput(key)) => handle_input(&mut scene, key),
            Some(Event::QuitGame) => break 'main,
            _ => {}
        };

        for py in 0.. term_size.1 {
            for px in 0.. term_size.0 {
                //Send out a ray
                let ray = scene.generate_ray(term_size, px, py);
                screen.set((px, py), scene.march(ray));
            }
        }

        screen.render();
    }

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Output("Thanks for using!"))?
        .execute(cursor::EnableBlinking)?
        .execute(ResetColor)?;

    Ok(())
}
