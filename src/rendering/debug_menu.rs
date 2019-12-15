// use crate::rendering::screen::Screen;
use crate::TerminalRaymarcher;

use crossterm::{
    style::Color,
};

pub struct DebugMenu {
    pub folded: bool,
    pub fps: f32,
    pub object_count: usize,

    pub bg_col: Color,
    pub fg_col: Color,
}

impl DebugMenu {
    pub fn new() -> DebugMenu {
        DebugMenu {
            folded: true,
            fps: 0.0,
            object_count: 0,

            bg_col: Color::Rgb{r: 30, g: 20, b: 50},
            fg_col: Color::Rgb{r: 120, g: 0, b: 255},
        }
    }

    pub fn update_fps(&mut self, fps: f32) {
        self.fps = fps;
    }

    pub fn update_obj_count(&mut self, object_count: usize) {
        self.object_count = object_count;
    }

    pub fn render(&self, tm: &mut TerminalRaymarcher) {
        if self.folded {
            tm.set((0,1), ('[', self.fg_col));
            tm.set((1,1), ('+', self.fg_col));
            tm.set((2,1), (']', self.fg_col));
            for ix in 0..14 {
                for iy in 1..5 {
                    tm.set_bg((ix,iy), Color::Reset);
                }
            }
            tm.set_bg((0,1), self.bg_col);
            tm.set_bg((1,1), self.bg_col);
            tm.set_bg((2,1), self.bg_col);

        } else {
            for ix in 0..14 {
                for iy in 1..5 {
                    tm.set_bg((ix,iy), self.bg_col);
                }
                tm.set((ix,1), ('-', self.fg_col));
                tm.set((ix,4), ('-', self.fg_col));

                tm.set((0,1), ('[', self.fg_col));
                tm.set((1,1), ('-', self.fg_col));
                tm.set((2,1), (']', self.fg_col));

                for (i, c) in format!("fps: {}", (self.fps * 100.0).floor() / 100.0).chars().enumerate() {
                    tm.set((i as u16, 2), (c, self.fg_col));
                }

                for (i, c) in format!("objs: {}", self.object_count).chars().enumerate() {
                    tm.set((i as u16, 3), (c, self.fg_col));
                }
            }
        }
    }
}
