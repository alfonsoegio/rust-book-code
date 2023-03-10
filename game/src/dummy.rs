use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;

const ORI_N : i32 = 0;
const ORI_S : i32 = 1;
const ORI_W : i32 = 2;
const ORI_E : i32 = 3;

pub struct Dummy {
    pub position: Point,
    pub speed: Point,
    pub scale: Point,
    pub size: Point,
    pub n_ori: i32,
    pub ori: i32,
    pub animation: i32,
    pub n_animation: i32,
    pub collided: bool,
    pub state: i32,
    pub transition: i32,
    pub active_transition_animation: i32,
    pub overlap_transition: bool,
    pub movement: fn(&mut Dummy)
}

pub fn change_position(o: &mut Dummy) {
    o.position.x += o.scale.x * o.speed.x;
    o.position.y += o.scale.y * o.speed.y;
}

pub fn arrow_management(o: &mut Dummy, event: sdl2::event::Event) {
    match event {
        Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
            o.ori = ORI_N;
            o.speed.y = -1;
        },
        Event::KeyDown { keycode: Some(Keycode::Down),repeat: false, .. } => {
            o.ori = ORI_S;
            o.speed.y = 1;
        },
        Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
            o.ori = ORI_W;
            o.speed.x = -1;
        },
        Event::KeyDown { keycode: Some(Keycode::Right),repeat: false, .. } => {
            o.ori = ORI_E;
            o.speed.x = 1;
        },
        Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
            o.speed.y = 0;
        },
        Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
            o.speed.y = 0;
        },
        Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
            o.speed.x = 0;
        }
        Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
            o.speed.x = 0;
        },
        _ => {}
    }
}

pub fn arrow_movement(o: &mut Dummy) {
    change_position(o);
    if o.speed.x != 0 || o.speed.y != 0 {
        o.animation += 1;
    }
    if o.position.x - o.scale.x * o.size.x / 2 <= 0 {
        o.position.x += o.scale.x + 1;
    }
    if o.position.x + o.scale.x * o.size.x / 2 >= SCREEN_WIDTH as i32 {
        o.position.x -= o.scale.x + 1;
    }
    if o.position.y - o.scale.y * o.size.y / 2 <= 0 {
        o.position.y += o.scale.y + 1;
    }
    if o.position.y + o.scale.y * o.size.y / 2 >= SCREEN_HEIGHT as i32 {
        o.position.y -= o.scale.y + 1;
    }
}

impl Default for Dummy {
    fn default() -> Self {
        Self {
            position: Point::new(100,100),
            speed: Point::new(0,0),
            scale: Point::new(3,3),
            size: Point::new(16,24),
            n_ori: 4,
            ori: 0,
            animation: 0,
            n_animation: 8,
            collided: false,
            state: 0,
            transition: -1,
            active_transition_animation: 0,
            overlap_transition: true,
            movement: arrow_movement
        }
    }
}
