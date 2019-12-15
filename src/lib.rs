pub const THREAD_COUNT: u16 = 8;

use std::io::stdout;
use std::thread;
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
    camera::Camera,
};

pub mod rendering;
use rendering::{
    screen::Screen,
    debug_menu::DebugMenu,
    raymarching::Ray,
};

extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

fn clamp(x: u16, a: u16, b: u16) -> u16 {
    if x < a { return a };
    if x > b { return b };
    return x;
}

fn generate_ray(camera_yaw: f32, term_size: (u16, u16), px: u16, py: u16) -> Ray {
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

pub struct TerminalRaymarcher {
    pub scene_originator: Scene,
    pub screen_arc: Arc<Mutex<Screen>>,
    pub _raw: Result<crossterm::screen::RawScreen>,
    pub reader: AsyncReader,
    pub term_size: (u16, u16),
    pub camera: Camera,
}

#[derive(Debug)]
pub enum Event {
    HandleInput(KeyEvent),
    HandleMouse(MouseEvent),
    QuitGame,
}

impl TerminalRaymarcher {
    pub fn new() -> Result<TerminalRaymarcher> {
        let term_size: (u16, u16) = terminal::size()?;

        input().enable_mouse_mode()?; //TODO: Error handling, make this optional

        Ok(TerminalRaymarcher {
            scene_originator: Scene::new(),
            screen_arc: Arc::new(Mutex::new(Screen::new(term_size))),
            _raw: crossterm::screen::RawScreen::into_raw_mode(),
            reader: input().read_async(),
            term_size: term_size,
            camera: Camera::new([0.0, 0.0, 0.0],0.0,0.0),
        })
    }

    //TODO: Reimplement as Drop trait
    pub fn quit(&self) -> Result<()> {
        stdout()
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Output("Thanks for using!\n\r"))?
            .execute(cursor::Show)?
            .execute(ResetColor)?;

        input().disable_mouse_mode()?; //TODO: Error handling, make this based on if mouse mode is enabled
        //self._raw should be dropped when self is dropped

        Ok(())
    }

    pub fn add_sdf(&mut self, sdf: SDF) -> usize {
        self.scene_originator.push_sdf(sdf)
    }

    pub fn update_rotation(&mut self, idx: usize, rotation: Vector3<f32>) {
        self.scene_originator.update_rotation(idx, rotation);
    }

    pub fn get_object_count(&self) -> usize {
        self.scene_originator.distance_fields.len()
    }

    pub fn next_event(&mut self) -> Option<Event> {
        if let Some(event) = self.reader.next() {
            match event {
                InputEvent::Keyboard(KeyEvent::Esc) => return Some(Event::QuitGame),
                InputEvent::Keyboard(key) => return Some(Event::HandleInput(key)),
                InputEvent::Mouse(mouse) => return Some(Event::HandleMouse(mouse)),
                _ => {}
            }
        }
        None
    }

    pub fn prepare(&self) -> Result<()> {
        stdout()
            .execute(SetForegroundColor(Color::Blue))?
            // .execute(Output(format!("========DEBUG========\nTerm size: [{}; {}]\n=====================\n", term_size.0, term_size.1)))?
            .execute(ResetColor)?
            .execute(cursor::Hide)?;

        Ok(())
    }

    pub fn set(&mut self, pos: (u16, u16), value: (char, Color)) {
        let screen = Arc::clone(&self.screen_arc);
        let mut screen_handle = screen.lock().unwrap();
        (*screen_handle).set(pos, value);
    }

    pub fn set_bg(&mut self, pos: (u16, u16), bg_col: Color) {
        let screen = Arc::clone(&self.screen_arc);
        let mut screen_handle = screen.lock().unwrap();
        (*screen_handle).set_bg(pos, bg_col);
    }

    pub fn flush(&mut self) {
        let screen = Arc::clone(&self.screen_arc);
        let mut screen_handle = screen.lock().unwrap();
        (*screen_handle).flush(self.term_size, (Color::Reset, Color::Reset));
    }

    pub fn render(&self) -> Result<()> {
        let scene = self.scene_originator.clone();

        let mut thread_width = self.term_size.0 / THREAD_COUNT;
        let mut handles = vec![];

        for tx in 0..THREAD_COUNT {
            let screen = Arc::clone(&self.screen_arc);

            let camera_yaw = self.camera.yaw;
            let scene_handle = scene.clone(); //Clone inside the thread loop, so we don't end up moving it

            //This needs to here to ensure we actually fill the entire screen
            if THREAD_COUNT * thread_width < self.term_size.0 {
                thread_width += self.term_size.0 - THREAD_COUNT * thread_width;
            }

            let term_size = self.term_size;

            let handle = thread::spawn(move || {
                for px in clamp(tx * thread_width, 0, term_size.0) .. clamp(tx * thread_width + thread_width, 0, term_size.0) {
                    for py in 0.. term_size.1 {
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

        // let screen = Arc::clone(&self.screen_arc);
        // let mut screen_handle = screen.lock().unwrap();
        //
        // (*screen_handle).render();

        Ok(())
    }

    pub fn display(&self) {
        let screen = Arc::clone(&self.screen_arc);
        let mut screen_handle = screen.lock().unwrap();

        (*screen_handle).render();
    }
}
