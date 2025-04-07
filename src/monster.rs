use notan::{
    app::{App, Graphics, Texture},
    draw::{Draw, DrawImages, DrawTransform},
    math::Vec2,
};

pub struct Monster {
    pub pos: Vec2,
    texture: Texture,
    pub activated: bool,
}

impl Monster {
    pub fn new(gfx: &mut Graphics, x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            texture: gfx
                .create_texture()
                .from_image(include_bytes!("assets/monster.png"))
                .build()
                .unwrap(),
            activated: false,
        }
    }

    pub fn update(&mut self, app: &mut App, player_pos: &Vec2, friend_found: bool) {
        if player_pos.y > 540.
            && player_pos.y < 680.
            && player_pos.x > 635.
            && player_pos.x < 1221.
            && friend_found
        {
            self.activated = true;
        }

        if self.activated {
            self.pos.x -= 15. * app.timer.delta_f32() * 60.;
        }
    }

    pub fn render(&self, player_pos: &Vec2, draw: &mut Draw) {
        if self.activated {
            draw.image(&self.texture).translate(self.pos.x, self.pos.y);
        }
    }
}
