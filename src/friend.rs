use notan::{
    app::{App, Graphics, Texture},
    draw::{Draw, DrawImages, DrawTransform},
    math::Vec2,
};

pub struct Friend {
    pub pos: Vec2,
    pub found: bool,
    pub show: bool,
    texture: Texture,
}

impl Friend {
    pub fn new(gfx: &mut Graphics, x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            found: false,
            show: false,
            texture: gfx
                .create_texture()
                .from_image(include_bytes!("assets/player.png"))
                .build()
                .unwrap(),
        }
    }

    pub fn update(&mut self, app: &mut App, player_pos: &Vec2) {
        if !self.found {
            if self.pos.distance(*player_pos) < 80. {
                self.found = true;
                self.show = true;
            }
        } else if self.pos.distance(*player_pos) > 70. {
            let dir = (player_pos - self.pos).normalize();

            self.pos += dir * app.timer.delta_f32() * 60. * 3.;
        }
    }

    pub fn render(&self, player_pos: &Vec2, draw: &mut Draw) {
        if !self.show {
            return;
        }

        let dir = (player_pos - self.pos).normalize();

        draw.image(&self.texture)
            .rotate_from((16., 16.), dir.to_angle())
            .translate(self.pos.x, self.pos.y);
    }
}
