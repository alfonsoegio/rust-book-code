extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use std::time::Duration;
use sdl2::image::LoadTexture;
use sdl2::rect::{Point, Rect};

const SWIDTH: u32 = 640;
const SHEIGHT: u32 = 480;
const BACKGROUND_PATH: &str = "./media/images/backgrounds/background.png";
const HEART_PATH: &str = "./media/images/icons/heart.png";

fn render_hearts(canvas: &mut Canvas<sdl2::video::Window>, texture_creator: &TextureCreator<sdl2::video::WindowContext>) -> Result<(), String> {
    let mut heart_texture = texture_creator.load_texture(HEART_PATH)?;
    heart_texture.set_alpha_mod(110);
    let heart_positions = [50, 80, 110, 140, 170];
    for heart_position in heart_positions {
        let point = Point::new(heart_position, 30);
        let rect = Rect::new(point.x - 15, point.y - 15, 30, 30);
        canvas.copy_ex(&heart_texture, None, rect, 0.0, point, false, false)?;
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Game", SWIDTH, SHEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator.load_texture(BACKGROUND_PATH)?;
    canvas.copy(&background_texture, None, None)?;
    render_hearts(&mut canvas, &texture_creator)?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
