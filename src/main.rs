use std::io::{stdout, Write};
use std::{thread, time};

use crossterm::{
    execute,
    terminal,
    cursor,
    input::{input, AsyncReader, InputEvent, KeyEvent, MouseEvent, MouseButton},
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
    HandleMouse(MouseEvent),
    QuitGame,
}

fn next_event(reader: &mut AsyncReader) -> Option<Event> {
    if let Some(event) = reader.next() {
        match event {
            InputEvent::Keyboard(KeyEvent::Esc) => return Some(Event::QuitGame),
            InputEvent::Keyboard(key) => return Some(Event::HandleInput(key)),
            InputEvent::Mouse(mouse) => return Some(Event::HandleMouse(mouse)),
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

fn handle_mouse(scene: &mut Scene, mouse: MouseEvent) {
    scene.camera.yaw += 45.0;
    if mouse == MouseEvent::Press(MouseButton::Left, 1, 1) {
        scene.camera.yaw += 45.0;
    }
}

fn main() -> Result<()> {
    let term_size: (u16, u16) = terminal::size()?;

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Output(format!("========DEBUG========\nTerm size: [{}; {}]\n=====================\n", term_size.0, term_size.1)))?
        .execute(ResetColor)?
        .execute(cursor::DisableBlinking)?;

    let mut screen = Screen::new(term_size);

    //Show a funky splashscreen
    for y in 0.. term_size.1 {
        for x in 0.. term_size.0 {
            let mut value = ' ';
            if x % 2 == y % 2 { value = '/' }
            if x % 8 == y % 8 { value = '\\' }
            if x == 0 || x == term_size.0 - 1 { value = '|' }
            if y == 0 || y == term_size.1 - 1 { value = '-' }

            // if (y as f32).sin() > (x as f32).tan() { value = '/' } else { value = ' ' }
            // if (x ^ y) % 2 == 0 {
            //     value = 'x';
            // }

            screen.set((x, y), (value, Color::Grey));
            screen.set_bg((x, y), Color::Black);
        }
    }
    for (i, c) in "loading assets...".chars().enumerate() {
        screen.set((i as u16, term_size.1 - 1), (c, Color::Grey));
    }
    screen.render();
    screen.flush(term_size, (Color::Reset, Color::Reset));
    thread::sleep(std::time::Duration::from_secs(3)); //TODO: Get rid of this, but we need this now to test the loading screen as we have nothing to load yet lol

    let header = "!== terminal_raymarcher v1.0 ";
    let mut idx = 0;
    for c in header.chars() {
        screen.set((idx, 0), (c, Color::Red));
        idx += 1;
        if idx >= term_size.0 {
            break;
        }
    }
    if idx < term_size.0 {
        for i in idx..term_size.0 {
            if i < term_size.0 - 1 {
                screen.set((i, 0), ('=', Color::Red));
            } else if i == term_size.0 - 1 {
                screen.set((i, 0), ('!', Color::Red));
            } else {
                break;
            }
        }
    }

    let _raw = crossterm::screen::RawScreen::into_raw_mode();
    let mut stdin = input().read_async();
    let _mouse_mode = input().enable_mouse_mode()?;

    let mut scene = Scene::new();
    let sphere = SDF::new_sphere([2.0, 0.0, 5.0], 1.0, [255, 0, 0]);
    scene.distance_fields.push(sphere);
    let plane = SDF::new_plane(-1.0, [255, 255, 255]);
    scene.distance_fields.push(plane);
    let cube = SDF::new_cube([-2.0, 0.0, 5.0], [0.5, 0.5, 0.5], [0, 0, 255]);
    scene.distance_fields.push(cube);

    //TODO: measure deltatime
    'main: loop {
        while let Some(event) = next_event(&mut stdin) {
            match event {
                Event::HandleInput(key) => handle_input(&mut scene, key),
                Event::HandleMouse(mouse) => handle_mouse(&mut scene, mouse),
                Event::QuitGame => break 'main,
                _ => {}
            };
        }

        //Start at 1 so we have a single line as a header
        for py in 1.. term_size.1 {
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
