use notan::{
    app::{App, Color},
    math::Vec2,
};

use crate::{RENDER_HEIGHT, RENDER_WIDTH, camera::Camera2D, get_bg_color};

pub struct Echo {
    pub pos: Vec2,
    pub dir: f32,
    pub hit: bool,
    pub hit_color: Color,
    pub no_find: bool,
    pub lifetime: f32,
}

impl Echo {
    pub fn new(x: f32, y: f32, dir: f32, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            dir,
            hit: false,
            hit_color: color,
            no_find: false,
            lifetime: 1.,
        }
    }

    pub fn update(&mut self, bytes: &Vec<u8>, app: &mut App, camera: &Camera2D) {
        if self.lifetime <= 0. {
            return;
        }

        if !self.hit && !self.no_find {
            self.pos += Vec2::from_angle(self.dir) * 6. * app.timer.delta_f32() * 60.;

            let x = self.pos.x * (camera.work_size.x / (RENDER_WIDTH / 0.5));
            let y = self.pos.y * (camera.work_size.y / (RENDER_HEIGHT / 0.5));

            let color = get_bg_color(bytes, x, y);
            if color != Color::BLACK {
                if color == Color::RED {
                    self.no_find = true;
                    self.hit_color = Color::TRANSPARENT;
                } else {
                    self.pos += Vec2::from_angle(self.dir)
                        * fastrand::i32(1..80) as f32
                        * app.timer.delta_f32()
                        * 60.;
                    self.hit = true;
                    self.hit_color = color;
                }
            }
        } else {
            self.lifetime -= 0.004 * app.timer.delta_f32() * 60.;
            self.hit_color.a = self.lifetime;
        }
    }
}
