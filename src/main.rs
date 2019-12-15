extern crate terminal_raymarcher;
use terminal_raymarcher::{
    TerminalRaymarcher,
    Event,
    engine::{
        distance_field::SDF,
        scene::Scene,
    },
    rendering::debug_menu::DebugMenu,
};

// use terminal_raymarcher::rendering::raymarching::Ray;
use std::io::stdout;
use std::thread;
use std::time::SystemTime;
use std::sync::{Mutex, Arc};

use crossterm::{
    terminal,
    cursor,
    input::{input, AsyncReader, InputEvent, KeyEvent, MouseEvent, MouseButton},
    ExecutableCommand,
    style::{Attribute, Color, SetForegroundColor, SetBackgroundColor, ResetColor},
    Output,
    Result
};

// pub mod engine;
// use engine::{
//     distance_field::*,
//     scene::Scene,
// };
//
// pub mod rendering;
// use rendering::{
//     screen::Screen,
//     debug_menu::DebugMenu,
//     raymarching::Ray,
// };

extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

const THREAD_COUNT: u16 = 8;

fn clamp(x: u16, a: u16, b: u16) -> u16 {
    if x < a { return a };
    if x > b { return b };
    return x;
}

// #[derive(Debug)]
// pub enum Event {
//     HandleInput(KeyEvent),
//     HandleMouse(MouseEvent),
//     QuitGame,
// }
//
// fn next_event(reader: &mut AsyncReader) -> Option<Event> {
//     if let Some(event) = reader.next() {
//         match event {
//             InputEvent::Keyboard(KeyEvent::Esc) => return Some(Event::QuitGame),
//             InputEvent::Keyboard(key) => return Some(Event::HandleInput(key)),
//             InputEvent::Mouse(mouse) => return Some(Event::HandleMouse(mouse)),
//             _ => {}
//         }
//     }
//     None
// }

fn handle_input(tm: &mut TerminalRaymarcher, key: KeyEvent) {
    if key == KeyEvent::Char('d') {
        tm.camera.yaw += 2.0;
    }
    if key == KeyEvent::Char('a') {
        tm.camera.yaw -= 2.0;
    }
}

//TODO: Look into this, for some reason I can only get it to work on linux
fn handle_mouse(debug_menu: &mut DebugMenu, mouse: MouseEvent) {
    if mouse == MouseEvent::Press(MouseButton::Left, 0, 1) ||
        mouse == MouseEvent::Press(MouseButton::Left, 1, 1) ||
        mouse == MouseEvent::Press(MouseButton::Left, 2, 1) {
        debug_menu.folded = !debug_menu.folded;
    }
}

fn main() -> Result<()> {
    let term_size: (u16, u16) = terminal::size()?;

    let mut tm = TerminalRaymarcher::new()?;
    tm.prepare()?;

    //Show a funky loading
    //TODO: Add a cool logo
    // for y in 0.. term_size.1 {
    //     for x in 0.. term_size.0 {
    //         let mut value = ' ';
    //         if x % 2 == y % 2 { value = '/' }
    //         if x % 8 == y % 8 { value = '\\' }
    //         if x == 0 || x == term_size.0 - 1 { value = '|' }
    //         if y == 0 || y == term_size.1 - 1 { value = '-' }
    //
    //         // if (y as f32).sin() > (x as f32).tan() { value = '/' } else { value = ' ' }
    //         // if (x ^ y) % 2 == 0 {
    //         //     value = 'x';
    //         // }
    //
    //         tm.set((x, y), (value, Color::Grey));
    //         tm.set_bg((x, y), Color::Black);
    //     }
    // }
    // for (i, c) in "loading assets...".chars().enumerate() {
    //     tm.set((i as u16, term_size.1 - 1), (c, Color::Grey));
    // }
    // tm.render();
    // tm.flush(term_size, (Color::Reset, Color::Reset));

    let plane = SDF::new_plane(-1.0, [255, 255, 255]);
    tm.add_sdf(plane);
    let sphere = SDF::new_sphere([2.0, 0.0, 5.0], 1.0, [255, 0, 0]);
    tm.add_sdf(sphere);

    let mut rot_x = 0.0;
    let mut rot_y = 0.0;
    let mut rot_z = 0.0;
    let torus = SDF::new_torus([-2.0, 0.0, 5.0], [1.0, 0.5], [0, 255, 0], [rot_x, rot_y, rot_z]);
    let torus_idx = tm.add_sdf(torus);

    let mut debug_menu = DebugMenu::new();
    let mut deltatime = 0.0; //In seconds
    let header = "!== terminal_raymarcher v1.0 ";

    'main: loop {
        let start = SystemTime::now();

        while let Some(event) = tm.next_event() {
            match event {
                Event::QuitGame => break 'main,
                Event::HandleInput(key) => handle_input(&mut tm, key),
                Event::HandleMouse(mouse) => handle_mouse(&mut debug_menu, mouse),
                _ => {}
            }
        }

        rot_x -= 100.5 * deltatime;
        rot_y -= 20.5 * deltatime;
        rot_z += 60.5 * deltatime;
        tm.update_rotation(torus_idx, [rot_x, rot_y, rot_z]);

        tm.render()?;
        debug_menu.render(&mut tm);
        let mut idx = 0;
        for c in header.chars() {
            tm.set((idx, 0), (c, Color::Red));
            idx += 1;
            if idx >= term_size.0 {
                break;
            }
        }
        if idx < term_size.0 {
            for i in idx..term_size.0 {
                if i < term_size.0 - 1 {
                    tm.set((i, 0), ('=', Color::Red));
                } else if i == term_size.0 - 1 {
                    tm.set((i, 0), ('!', Color::Red));
                } else {
                    break;
                }
            }
        }

        tm.display();

        let deltatime_ms = start.elapsed().expect("Time went backwards!!").as_millis();
        deltatime = (deltatime_ms as f32) / 1000.0;
        debug_menu.update_fps(1.0 / deltatime);
        debug_menu.update_obj_count(tm.get_object_count())
    }

    tm.quit()?;

    Ok(())
}
