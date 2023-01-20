use sdl2::rect::Point;

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
    pub movement: fn(&mut Dummy, sdl2::event::Event)
}

pub fn default_movement(dummy: &mut Dummy, _event: sdl2::event::Event) {
    println!("Moving: {:?}", dummy.position);
    dummy.position.x += 10;
}

impl Default for Dummy {
    fn default() -> Self {
        Self {
            position: Point::new(10,10),
            speed: Point::new(0,0),
            scale: Point::new(3,3),
            size: Point::new(24,16),
            n_ori: 4,
            ori: 0,
            animation: 0,
            n_animation: 0,
            collided: false,
            state: 0,
            transition: -1,
            active_transition_animation: 0,
            overlap_transition: true,
            movement: default_movement
        }
    }
}
