extern crate sdl2;

// use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
// use sdl2::rect::Point;
use std::time::Duration;

mod stage;
mod dummy;

use dummy::Dummy;

const SWIDTH: u32 = 640;
const SHEIGHT: u32 = 480;
const _HERO_PATH: &str = "./media/images/dummies/link1.png";

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game", SWIDTH, SHEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    let texture_creator = canvas.texture_creator();
    let canvas = stage::render_stage(&mut canvas, &texture_creator, 0)?;
    canvas.present();

    // ::std::thread::sleep(Duration::new(0, 2_000_000_000u32));
    // let canvas = stage::render_stage(canvas, &texture_creator, 666)?;

    let hero: &mut Dummy = &mut dummy::Dummy::default();

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
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => (hero.movement)(hero, event),
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
