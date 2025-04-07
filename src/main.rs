pub mod beacon;
pub mod camera;
pub mod echo;
pub mod friend;
pub mod monster;
pub mod player;

use beacon::Beacon;
use camera::Camera2D;
use echo::Echo;
use friend::Friend;
use monster::Monster;
use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use player::Player;

const RENDER_WIDTH: f32 = 320.;
const RENDER_HEIGHT: f32 = 180.;

#[derive(PartialEq, Eq)]
pub enum Scene {
    Start,
    Game,
    End,
}

struct DeathScene {
    pub fade: f32,
    pub show_text: bool,
}

impl DeathScene {
    pub fn new() -> Self {
        Self {
            fade: 0.,
            show_text: false,
        }
    }
}

struct SoundSystem {
    pub sound_effects: [AudioSource; 8],
    sound: [Option<Sound>; 8],
    repeat: [bool; 8],
    volume: [f32; 8],
}

impl SoundSystem {
    pub fn new(app: &mut App) -> Self {
        let m0 = app
            .audio
            .create_source(include_bytes!("assets/ambiance.mp3"))
            .unwrap();
        let m1 = app
            .audio
            .create_source(include_bytes!("assets/jumpscare.mp3"))
            .unwrap();
        let m2 = app
            .audio
            .create_source(include_bytes!("assets/creepy_cave.mp3"))
            .unwrap();
        let m3 = app
            .audio
            .create_source(include_bytes!("assets/monster.mp3"))
            .unwrap();
        let m4 = app
            .audio
            .create_source(include_bytes!("assets/echo_scary.wav"))
            .unwrap();
        let m5 = app
            .audio
            .create_source(include_bytes!("assets/more_scary.wav"))
            .unwrap();
        let m6 = app
            .audio
            .create_source(include_bytes!("assets/scary_sound.mp3"))
            .unwrap();
        let m7 = app
            .audio
            .create_source(include_bytes!("assets/short_scary.wav"))
            .unwrap();

        Self {
            sound_effects: [m0, m1, m2, m3, m4, m5, m6, m7],
            sound: [None, None, None, None, None, None, None, None],
            repeat: [true, false, false, false, false, false, false, false],
            volume: [1., 1., 1., 1., 1., 1., 1., 1.],
        }
    }
}

#[derive(AppState)]
struct State {
    camera: Camera2D,
    player_world_rtex: RenderTexture,
    cave_texture: Texture,
    player: Player,
    cave_bytes: Vec<u8>,
    echoes: Vec<Echo>,
    beacons: Vec<Beacon>,
    friend: Friend,
    num_of_beacons: i32,
    font: Font,
    monster: Monster,
    scene: Scene,
    death_scene: DeathScene,
    sound_system: SoundSystem,
    music_start: bool,
    music_delay: i32,
    played_random_time: i32,
    show_found_text: bool,
    found_text_timer: f32,
    show_hint: bool,
    show_hint_timer: f32,
}

fn play_music(index: usize, app: &mut App, sound_system: &mut SoundSystem) {
    let sound = app.audio.play_sound(
        &sound_system.sound_effects[index],
        sound_system.volume[index],
        sound_system.repeat[index],
    );
    sound_system.sound[index] = Some(sound);
}

// fn is_playing(index: usize, app: &mut App, sound_system: &mut SoundSystem) -> bool {
//     match &sound_system.sound[index] {
//         Some(s) => !app.audio.is_stopped(s),
//         None => false,
//     }
// }

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(
            WindowConfig::new()
                .set_vsync(true)
                .set_size(1440, 810)
                .set_multisampling(0),
        )
        .add_config(log::LogConfig::debug())
        .update(update)
        .draw(draw)
        .add_config(DrawConfig)
        .build()
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let mut camera = Camera2D::new(0., 0., RENDER_WIDTH * 2., RENDER_HEIGHT * 2.);
    camera.set_zoom(1.);

    let sound_system = SoundSystem::new(app);

    let cave_texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/cave.png"))
        .with_filter(TextureFilter::Nearest, TextureFilter::Nearest)
        .build()
        .unwrap();

    let cave_bytes = load_bytes(&cave_texture, gfx);

    State {
        camera,
        player_world_rtex: gfx
            .create_render_texture(RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
            .with_filter(TextureFilter::Nearest, TextureFilter::Nearest)
            .build()
            .unwrap(),
        cave_texture,
        player: Player::new(gfx),
        cave_bytes,
        echoes: vec![],
        beacons: vec![
            Beacon::new(885., 165., false, 15),
            Beacon::new(880., 625., false, 15),
            Beacon::new(227., 143., false, 15),
            Beacon::new(1803., 143., false, 15),
        ],
        friend: Friend::new(gfx, 1825., 1080.),
        num_of_beacons: 3,
        font: gfx
            .create_font(include_bytes!("assets/slkscr.ttf"))
            .unwrap(),
        monster: Monster::new(gfx, 1268., 460.),
        scene: Scene::Start,
        death_scene: DeathScene::new(),
        sound_system,
        music_start: false,
        music_delay: 0,
        played_random_time: 0,
        show_found_text: false,
        found_text_timer: 250.,
        show_hint: false,
        show_hint_timer: 250.,
    }
}

fn update(app: &mut App, state: &mut State) {
    if state.music_start {
        state.music_delay += 1;
    }

    if state.music_delay == 20 {
        play_music(0, app, &mut state.sound_system);
        state.music_delay = 0;
        state.music_start = false;
    }

    if app.timer.elapsed_f32() as i32 % 30 == 0
        && state.scene == Scene::Game
        && state.played_random_time != app.timer.elapsed_f32() as i32
    {
        if fastrand::bool() {
            play_music(fastrand::i32(2..8) as usize, app, &mut state.sound_system);
        }

        state.played_random_time = app.timer.elapsed_f32() as i32;
    }

    if state.player.pos.distance(Vec2::new(900., 1600.)) < 100. {
        state.show_hint = true;
    }

    state.player.update(
        app,
        &state.cave_bytes,
        &mut state.echoes,
        &state.camera,
        &state.scene,
    );

    if state.monster.activated {
        if state.scene != Scene::End {
            play_music(1, app, &mut state.sound_system);
        }
        state.scene = Scene::End;
    }

    state
        .camera
        .set_position(state.player.pos.x + 16., state.player.pos.y + 16.);

    if !state.echoes.is_empty() {
        for echo in &mut state.echoes {
            echo.update(&state.cave_bytes, app, &state.camera);

            if !echo.hit {
                if echo.pos.distance(state.friend.pos) < 16. {
                    state.friend.show = true;
                }
            }
        }
    }

    if state.friend.found {
        state.show_found_text = true;
    }

    let mut i = 0;
    while i < state.echoes.len() {
        if state.echoes[i].lifetime <= 0. {
            state.echoes.remove(i);
            i -= 1;
        }
        i += 1;
    }

    state
        .monster
        .update(app, &state.player.pos, state.friend.found);

    state.friend.update(app, &state.player.pos);

    if app.keyboard.was_pressed(KeyCode::B) && state.num_of_beacons > 0 {
        state.beacons.push(Beacon::new(
            state.player.pos.x,
            state.player.pos.y,
            true,
            60,
        ));
        state.num_of_beacons -= 1;
    }

    for beacon in &mut state.beacons {
        beacon.update(&mut state.echoes, app.timer.delta_f32());
    }

    if state.scene == Scene::End {
        if state.player.pos.distance(state.monster.pos) < 400. {
            state.death_scene.fade =
                map(state.monster.pos.x - state.player.pos.x, 0., 400., 1., 0.);
        }

        if state.monster.pos.x < state.player.pos.x {
            state.death_scene.fade = 1.;
        }

        if state.monster.pos.x < -600. {
            state.death_scene.show_text = true;
        }
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut player_draw = state.player_world_rtex.create_draw();
    player_draw.clear(Color::TRANSPARENT);

    state.camera.apply(&mut player_draw);

    // player_draw.image(&state.cave_texture);
    state.monster.render(&state.player.pos, &mut player_draw);

    for beacon in &state.beacons {
        if beacon.visible {
            player_draw
                .circle(10.)
                .position(beacon.pos.x + 20., beacon.pos.y + 20.)
                .stroke_color(Color::from_rgba(0.8, 0.0, 0.8, 1.0))
                .stroke(4.);
        }
    }

    if state.scene != Scene::Start {
        state.player.render(&mut player_draw);
    }

    state.friend.render(&state.player.pos, &mut player_draw);

    for echo in &state.echoes {
        if echo.pos.x > state.camera.pos.x - RENDER_WIDTH
            && echo.pos.x < state.camera.pos.x + RENDER_WIDTH
            && echo.pos.y > state.camera.pos.y - RENDER_HEIGHT
            && echo.pos.y < state.camera.pos.y + RENDER_HEIGHT
        {
            player_draw
                .rect((echo.pos.x, echo.pos.y), (5., 5.))
                .fill_color(echo.hit_color);
        }
    }

    player_draw.transform().pop();

    gfx.render_to(&state.player_world_rtex, &player_draw);

    let mut draw = gfx.create_draw();

    draw.image(&state.player_world_rtex.texture())
        .size(app.window().width() as f32, app.window().height() as f32);

    // if state.scene == Scene::Game {
    //     draw.text(
    //         &state.font,
    //         &format!("Beacons left: {}", state.num_of_beacons),
    //     )
    //     .size(40.)
    //     .color(Color::WHITE);
    // }
    if state.show_found_text && state.found_text_timer > 0. {
        draw.text(
            &state.font,
            "Thank god you found me.. Please lead me back..",
        )
        .size(40.)
        .color(Color::WHITE)
        .h_align_center()
        .v_align_middle()
        .position(
            app.window().width() as f32 / 2.,
            app.window().height() as f32 / 1.1,
        );

        state.found_text_timer -= app.timer.delta_f32() * 60.;
    }

    if state.show_hint && state.show_hint_timer > 0. {
        draw.text(
            &state.font,
            "There's some white debree left.. It must be this way",
        )
        .size(40.)
        .color(Color::WHITE)
        .h_align_center()
        .v_align_middle()
        .position(
            app.window().width() as f32 / 2.,
            app.window().height() as f32 / 1.1,
        );

        state.show_hint_timer -= app.timer.delta_f32() * 60.;
    }

    if state.scene == Scene::Start {
        draw.rect(
            (0., 0.),
            (app.window().width() as f32, app.window().height() as f32),
        )
        .color(Color::from_bytes(0, 0, 0, 100));

        let c = if app.mouse.x > app.window().width() as f32 / 4. - 120.
            && app.mouse.x < app.window().width() as f32 / 4. + 120.
            && app.mouse.y > app.window().height() as f32 / 2. - 30.
            && app.mouse.y < app.window().height() as f32 / 2. + 30.
        {
            if app.mouse.left_was_pressed() {
                state.music_start = true;
                state.scene = Scene::Game;
            }
            Color::from_rgba(1., 1., 1., 0.25)
        } else {
            Color::TRANSPARENT
        };

        draw.rect(
            (
                app.window().width() as f32 / 4. - 120.,
                app.window().height() as f32 / 2. - 30.,
            ),
            (240., 60.),
        )
        .color(c);

        draw.text(&state.font, "START")
            .size(40.)
            .color(Color::WHITE)
            .h_align_center()
            .v_align_middle()
            .position(
                app.window().width() as f32 / 4.,
                app.window().height() as f32 / 2.,
            );

        draw.text(&state.font, "Blind depths")
            .size(70.)
            .color(Color::WHITE)
            .h_align_center()
            .v_align_middle()
            .position(
                app.window().width() as f32 / 2.,
                app.window().height() as f32 / 10.,
            );

        draw.text(&state.font, "STORY:\nYou're a submarine pilot in one of the deepest parts of the ocean and your only form of navigation is echoes you send that reveal the details of the cave walls. Your colleague got lost in one of the most complex deep ocean cave systems. Countless have already gone missing in that cave.\nRumors say that some kind of creature lives there..\n\nFind him and bring him back.\n\n\n\n\nControls:\nUse A and D keys to turn\nUse the W key to accelerate\nUse Space to send an echo\nUse the B key to place beacons (You only have 3)\n*Beacons are useful for navigation and marking areas")
            .size(22.)
            .color(Color::WHITE)
            .h_align_left()
            .v_align_middle()
            .max_width(app.window().width() as f32 / 2.5)
            .position(
                app.window().width() as f32 / 2.,
                app.window().height() as f32 / 2.,
            );
    }

    if state.scene == Scene::End {
        draw.rect(
            (0., 0.),
            (app.window().width() as f32, app.window().height() as f32),
        )
        .color(Color::from_rgba(0., 0., 0., state.death_scene.fade));

        if state.death_scene.show_text {
            draw.text(&state.font, "The end...")
                .size(40.)
                .color(Color::WHITE)
                .h_align_center()
                .v_align_middle()
                .position(
                    app.window().width() as f32 / 2.,
                    app.window().height() as f32 / 2.,
                );
        }
    }

    gfx.render(&draw);
}

pub fn load_bytes(texture: &Texture, gfx: &mut Graphics) -> Vec<u8> {
    let bpp = texture.format().bytes_per_pixel() as usize;
    let width = texture.width() as usize;
    let height = texture.height() as usize;
    let len = width * height * bpp;

    let mut bytes = vec![0; len];
    gfx.read_pixels(texture).read_to(&mut bytes).unwrap();

    bytes
}

pub fn get_bg_color(bytes: &Vec<u8>, x: f32, y: f32) -> Color {
    let xi = x.floor() as usize;
    let yi = y.floor() as usize;

    if xi >= 2017 || yi >= 2216 {
        return Color::new(0.0, 0.0, 0.0, 0.0); // Transparent or error default
    }

    let index = (yi * 2017 + xi) * 4;

    let r = bytes[index] as f32 / 255.0;
    let g = bytes[index + 1] as f32 / 255.0;
    let b = bytes[index + 2] as f32 / 255.0;
    let a = bytes[index + 3] as f32 / 255.0;

    Color::new(r, g, b, a)
}

pub fn send_echo(echoes: &mut Vec<Echo>, pos: &Vec2, dir: f32, color: Color) {
    for i in 0..=60 {
        let echo = Echo::new(
            pos.x + 16.,
            pos.y + 16.,
            (i as f32 * 6.).to_radians() - dir,
            color,
        );

        echoes.push(echo);
    }
}

pub fn map(value: f32, begin: f32, end: f32, new_begin: f32, new_end: f32) -> f32 {
    new_begin + (new_end - new_begin) * ((value - begin) / (end - begin))
}
