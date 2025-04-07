use notan::{
    app::{App, Color, Graphics, Texture},
    draw::{Draw, DrawImages, DrawShapes, DrawTransform},
    math::Vec2,
};

use crate::{
    RENDER_HEIGHT, RENDER_WIDTH, Scene, camera::Camera2D, echo::Echo, get_bg_color, send_echo,
};

pub struct Player {
    texture: Texture,
    pub pos: Vec2,
    vel: Vec2,
    dir: f32,
}

impl Player {
    pub fn new(gfx: &mut Graphics) -> Self {
        Self {
            texture: gfx
                .create_texture()
                .from_image(include_bytes!("assets/player.png"))
                .build()
                .unwrap(),
            pos: Vec2::new(201., 167.),
            vel: Vec2::ZERO,
            dir: 0.,
        }
    }

    pub fn update(
        &mut self,
        app: &mut App,
        bytes: &Vec<u8>,
        echoes: &mut Vec<Echo>,
        camera: &Camera2D,
        scene: &Scene,
    ) {
        if app.keyboard.is_down(notan::prelude::KeyCode::W) && *scene == Scene::Game {
            self.vel += Vec2::from_angle(self.dir.to_radians()) * app.timer.delta_f32() * 1.;
        }
        self.vel = self.vel.clamp_length_max(3.);
        self.pos += self.vel;

        self.vel -= (self.vel / 100.) * app.timer.delta_f32() * 60.;

        if app.keyboard.is_down(notan::prelude::KeyCode::D) && *scene == Scene::Game {
            self.dir += app.timer.delta_f32() * 60.;
        }
        if app.keyboard.is_down(notan::prelude::KeyCode::A) && *scene == Scene::Game {
            self.dir -= app.timer.delta_f32() * 60.;
        }

        if app.keyboard.was_pressed(notan::prelude::KeyCode::Space) && *scene == Scene::Game {
            send_echo(echoes, &self.pos, self.dir, Color::PURPLE);
        }

        if get_bg_color(
            bytes,
            self.pos.x * (camera.work_size.x / (RENDER_WIDTH / 0.5)) + 16.,
            self.pos.y * (camera.work_size.y / (RENDER_HEIGHT / 0.5)) + 16.,
        ) != Color::BLACK
        {
            self.vel *= -1.;
        }
    }

    pub fn render(&mut self, draw: &mut Draw) {
        draw.image(&self.texture)
            .rotate_degrees_from((16., 16.), self.dir)
            .translate(self.pos.x, self.pos.y);

        for i in 0..=30 {
            let angle = Vec2::from_angle((i as f32 * 12.).to_radians() - self.dir) * 40.;

            draw.rect((angle.x, angle.y), (2., 2.))
                .color(Color::PURPLE)
                .translate(self.pos.x + 16., self.pos.y + 16.);
        }
    }
}
