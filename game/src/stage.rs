use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;

const HEART_PATH: &str = "./media/images/icons/heart.png";
const BACKGROUND_PATH: &str = "./media/images/backgrounds/background.png";
const FONT_PATH: &str = "./media/font/FreeSerifBold.ttf";

// The point here: https://github.com/Rust-SDL2/rust-sdl2/issues/1105#issuecomment-830083543
//   Put the canvas into the stack: "If you want to use a Canvas in multiple places,
//   the "Rust way" would be to not have it owned or borrowed in multiple places,
//   but rather put it on the stack, and pass it through every call where it's needed"

pub fn render_background<'a>(canvas: &'a mut Canvas<sdl2::video::Window>,
                             texture_creator: &'a TextureCreator<sdl2::video::WindowContext>)
                             -> Result<&'a mut Canvas<sdl2::video::Window>, String> {
    canvas.clear();
    let background_texture = texture_creator.load_texture(BACKGROUND_PATH)?;
    canvas.copy(&background_texture, None, None)?;
    Ok(canvas)
}

pub fn render_hearts<'a>(canvas: &'a mut Canvas<sdl2::video::Window>,
                         texture_creator: &'a TextureCreator<sdl2::video::WindowContext>)
                         -> Result<&'a mut Canvas<sdl2::video::Window>, String> {
    let mut heart_texture = texture_creator.load_texture(HEART_PATH)?;
    heart_texture.set_alpha_mod(110);
    let heart_positions = [50, 80, 110, 140, 170];
    for heart_position in heart_positions {
        let point = Point::new(heart_position, 30);
        let rect = Rect::new(point.x - 15, point.y - 15, 30, 30);
        canvas.copy_ex(&heart_texture, None, rect, 0.0, point, false, false)?;
    }
    Ok(canvas)
}

pub fn render_score<'a>(canvas: &'a mut Canvas<sdl2::video::Window>,
                        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
                        score: i32) -> Result<&'a mut Canvas<sdl2::video::Window>, String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font(FONT_PATH, 24)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let score_label = format!("Score : {}", score);
    let surface = font.render(&score_label).blended(Color::RGBA(0, 0, 0, 200)).map_err(|e| e.to_string())?;
    let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
    let point = Point::new(450, 0);
    let rect = Rect::new(point.x, point.y, 14 * (score_label.len() as u32), 60);
    canvas.copy(&texture, None, Some(rect))?;
    Ok(canvas)
}

pub fn render_stage<'a>(canvas: &'a mut Canvas<sdl2::video::Window>,
                        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
                        score: i32) -> Result<&'a mut Canvas<sdl2::video::Window>, String> {
    let canvas: &mut Canvas<sdl2::video::Window> = render_background(canvas, texture_creator)?;
    let canvas: &mut Canvas<sdl2::video::Window>  = render_hearts(canvas, texture_creator)?;
    let canvas: &mut Canvas<sdl2::video::Window>  = render_score(canvas, texture_creator, score)?;
    Ok(canvas)
}
