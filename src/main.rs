use crate::rendering::raymarching::Ray;
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

pub mod engine;
use engine::{
    distance_field::*,
    scene::Scene,
};

pub mod rendering;
use rendering::{
    screen::Screen,
    debug_menu::DebugMenu,
};

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

//TODO: Look into this, for some reason I can only get it to work on linux
fn handle_mouse(debug_menu: &mut DebugMenu, mouse: MouseEvent) {
    if mouse == MouseEvent::Press(MouseButton::Left, 0, 1) ||
        mouse == MouseEvent::Press(MouseButton::Left, 1, 1) ||
        mouse == MouseEvent::Press(MouseButton::Left, 2, 1) {
        debug_menu.folded = !debug_menu.folded;
    }
}

pub fn generate_ray(camera_yaw: f32, term_size: (u16, u16), px: u16, py: u16) -> Ray {
    let fc = ((term_size.0 - px) as f32, (term_size.1 - py) as f32);
    let p = ((-(term_size.0 as f32) + 2.0 * fc.0) / (term_size.1 as f32), (-(term_size.1 as f32) + 2.0 * fc.1) / (term_size.1 as f32));
    let mut ray = Ray::new([0.0, 0.0, 0.0], vmath::vec3_normalized([p.0 * 0.5, p.1, 2.0]));

    let r = camera_yaw / 180.0 * 3.14;
    let dx = ray.direction[0] * r.cos() - ray.direction[2] * r.sin();
    let dy = ray.direction[2] * r.cos() + ray.direction[0] * r.sin();
    ray.direction[0] = -dx;
    ray.direction[2] = dy;

    return ray;
}

fn main() -> Result<()> {
    let term_size: (u16, u16) = terminal::size()?;

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Output(format!("========DEBUG========\nTerm size: [{}; {}]\n=====================\n", term_size.0, term_size.1)))?
        .execute(ResetColor)?
        .execute(cursor::DisableBlinking)?;

    // let mut screen = Screen::new(term_size);
    let mut screen_arc = Arc::new(Mutex::new(Screen::new(term_size)));
    let mut debug_menu = DebugMenu::new();
    let mut deltatime = 0.0; //In seconds

    let screen = Arc::clone(&screen_arc);
    {
        let mut screen_handle = screen.lock().unwrap();

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

                (*screen_handle).set((x, y), (value, Color::Grey));
                (*screen_handle).set_bg((x, y), Color::Black);
            }
        }
        for (i, c) in "loading assets...".chars().enumerate() {
            (*screen_handle).set((i as u16, term_size.1 - 1), (c, Color::Grey));
        }
        (*screen_handle).render();
        (*screen_handle).flush(term_size, (Color::Reset, Color::Reset));
        thread::sleep(std::time::Duration::from_secs(0)); //TODO: Get rid of this, but we need this now to test the loading screen as we have nothing to load yet lol

        let header = "!== terminal_raymarcher v1.0 ";
        let mut idx = 0;
        for c in header.chars() {
            (*screen_handle).set((idx, 0), (c, Color::Red));
            idx += 1;
            if idx >= term_size.0 {
                break;
            }
        }
        if idx < term_size.0 {
            for i in idx..term_size.0 {
                if i < term_size.0 - 1 {
                    (*screen_handle).set((i, 0), ('=', Color::Red));
                } else if i == term_size.0 - 1 {
                    (*screen_handle).set((i, 0), ('!', Color::Red));
                } else {
                    break;
                }
            }
        }
    }

    let _raw = crossterm::screen::RawScreen::into_raw_mode();
    let mut stdin = input().read_async();
    input().enable_mouse_mode()?;

    let mut scene_originator = Scene::new();
    let plane = SDF::new_plane(-1.0, [255, 255, 255]);
    scene_originator.push_sdf(plane);
    // scene.distance_fields.push(plane);

    let sphere = SDF::new_sphere([2.0, 0.0, 5.0], 1.0, [255, 0, 0]);
    scene_originator.push_sdf(sphere);
    // scene.distance_fields.push(sphere);
    //position, radius/inner radius, colour, rotation
    let mut rot_x = 0.0;
    let mut rot_y = 0.0;
    let mut rot_z = 0.0;
    let torus = SDF::new_torus([-2.0, 0.0, 5.0], [1.0, 0.5], [0, 255, 0], [rot_x, rot_y, rot_z]);
    scene_originator.push_sdf(torus);
    // scene.distance_fields.push(torus);
    // let cube = SDF::new_cube([-2.0, 0.0, 5.0], [0.5, 0.5, 0.5], [0, 0, 255]);
    // scene.distance_fields.push(cube);

    let mut scene = scene_originator.clone();

    //TODO: measure deltatime
    'main: loop {
        let start = SystemTime::now(); //TODO: check if using Instant isn't better

        while let Some(event) = next_event(&mut stdin) {
            match event {
                Event::HandleInput(key) => handle_input(&mut scene, key),
                Event::HandleMouse(mouse) => handle_mouse(&mut debug_menu, mouse),
                Event::QuitGame => break 'main,
                _ => {} //Technically doesn't get used at all, but it allows us to implement future events without writing code here
            };
        }

        let mut thread_width = term_size.0 / THREAD_COUNT;
        let mut handles = vec![];

        for tx in 0..THREAD_COUNT {
            let screen = Arc::clone(&screen_arc);

            let camera_yaw = scene.camera.yaw;
            let scene_handle = scene.clone();

            //This needs to here to ensure we actually fill the entire screen
            if THREAD_COUNT * thread_width < term_size.0 {
                thread_width += term_size.0 - THREAD_COUNT * thread_width;
            }

            let handle = thread::spawn(move || {
                for px in clamp(tx * thread_width, 0, term_size.0) .. clamp(tx * thread_width + thread_width, 0, term_size.0) {
                    for py in 1..term_size.1 {
                        let ray = generate_ray(camera_yaw, term_size, px, py);
                        let ray_result = scene_handle.march(ray);
                        let mut screen_handle = screen.lock().unwrap();
                        (*screen_handle).set((px, py), ray_result);
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        rot_x -= 100.5 * deltatime;
        rot_y -= 20.5 * deltatime;
        rot_z += 60.5 * deltatime;
        // scene.distance_fields[2].update_rotation([rot_x, rot_y, rot_z]);
        scene.update_rotation(2, [rot_x, rot_y, rot_z]);

        let screen = Arc::clone(&screen_arc);
        let mut screen_handle = screen.lock().unwrap();
        debug_menu.render(&mut (*screen_handle));

        (*screen_handle).render();

        let deltatime_ms = start.elapsed().expect("Time went backwards!!").as_millis();
        deltatime = (deltatime_ms as f32) / 1000.0;
        debug_menu.update_fps(1.0 / deltatime);
        // debug_menu.update_obj_count(scene.distance_fields.len());
    }

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Output("Thanks for using!\n\r"))?
        .execute(cursor::EnableBlinking)?
        .execute(ResetColor)?;

    input().disable_mouse_mode()?;

    Ok(())
}
