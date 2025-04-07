use notan::{app::Color, math::Vec2};

use crate::{echo::Echo, send_echo};

pub struct Beacon {
    pub pos: Vec2,
    pub visible: bool,
    freq: i32,
    timer: f32,
}

impl Beacon {
    pub fn new(x: f32, y: f32, visible: bool, freq: i32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            visible,
            freq,
            timer: 0.,
        }
    }

    pub fn update(&mut self, echoes: &mut Vec<Echo>, dt: f32) {
        self.timer += dt * 60.;

        if self.timer > self.freq as f32 {
            self.timer = 0.;

            let color = if self.visible {
                Color::PURPLE
            } else {
                Color::TRANSPARENT
            };

            send_echo(echoes, &self.pos, fastrand::f32(), color);
        }
    }
}
