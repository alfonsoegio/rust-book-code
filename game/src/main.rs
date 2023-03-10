extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod stage;
mod dummy;

use dummy::Dummy;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const FONT_PATH: &str = "./media/font/FreeSerifBold.ttf";

const HEART_PATH: &str = "./media/images/icons/heart.png";
const BACKGROUND_PATH: &str = "./media/images/backgrounds/background.png";
const MAIN_SPEED: u32 = 200;

const HERO_PATH: &str = "./media/images/dummies/link1.png";

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator.load_texture(BACKGROUND_PATH)?;
    let heart_texture = texture_creator.load_texture(HEART_PATH)?;

    let canvas = stage::render_stage(
        &mut canvas,
        &texture_creator,
        &background_texture,
        &heart_texture, 0)?;

    // ::std::thread::sleep(Duration::new(0, 2_000_000_000u32));
    // let canvas = stage::render_stage(canvas, &texture_creator, 666)?;

    let hero: &mut Dummy = &mut dummy::Dummy::default();
    let hero_texture = texture_creator.load_texture(HERO_PATH)?;
    let src = Rect::new(0, 0, hero.size.x as u32, hero.size.y as u32);
    let dst = Rect::from_center(hero.position, 3 * (hero.size.x as u32), 3 * (hero.size.y as u32));
    canvas.copy_ex(&hero_texture, src, dst, 0.0, None, false, false)?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                _ => {
                    dummy::arrow_management(hero, event);

                }
            }
        }
        let canvas = stage::render_stage(
            canvas,
            &texture_creator,
            &background_texture,
            &heart_texture, 0)?;
        let src_ox = hero.ori * hero.size.x;
        let src_oy = (hero.animation % hero.n_animation) * hero.size.y;
        let src = Rect::new(src_ox, src_oy, hero.size.x as u32, hero.size.y as u32);
        let dst = Rect::from_center(
            hero.position,
            (hero.scale.x as u32) * (hero.size.x as u32),
            (hero.scale.y as u32) * (hero.size.y as u32));
        canvas.copy_ex(&hero_texture, src, dst, 0.0, None, false, false)?;
        (hero.movement)(hero);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / MAIN_SPEED));
    };
    Ok(())
}
