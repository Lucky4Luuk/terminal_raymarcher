use crate::rendering::screen::Screen;

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

            bg_col: Color::Rgb{r: 50, g: 20, b: 20},
            fg_col: Color::Rgb{r: 255, g: 0, b: 0},
        }
    }

    pub fn update_fps(&mut self, fps: f32) {
        self.fps = fps;
    }

    pub fn update_obj_count(&mut self, object_count: usize) {
        self.object_count = object_count;
    }

    pub fn render(&self, screen: &mut Screen) {
        if self.folded {
            screen.set((0,1), ('[', self.fg_col));
            screen.set((1,1), ('+', self.fg_col));
            screen.set((2,1), (']', self.fg_col));
            for ix in 0..14 {
                for iy in 1..5 {
                    screen.set_bg((ix,iy), Color::Reset);
                }
            }
            screen.set_bg((0,1), self.bg_col);
            screen.set_bg((1,1), self.bg_col);
            screen.set_bg((2,1), self.bg_col);

        } else {
            for ix in 0..14 {
                for iy in 1..5 {
                    screen.set_bg((ix,iy), self.bg_col);
                }
                screen.set((ix,1), ('-', self.fg_col));
                screen.set((ix,4), ('-', self.fg_col));

                screen.set((0,1), ('[', self.fg_col));
                screen.set((1,1), ('-', self.fg_col));
                screen.set((2,1), (']', self.fg_col));

                for (i, c) in format!("fps: {}", (self.fps * 100.0).floor() / 100.0).chars().enumerate() {
                    screen.set((i as u16, 2), (c, self.fg_col));
                }

                for (i, c) in format!("objs: {}", self.object_count).chars().enumerate() {
                    screen.set((i as u16, 3), (c, self.fg_col));
                }
            }
        }
    }
}
