use macroquad::prelude::*;

pub static WINDOW_SIZE_PX: Vec2 = Vec2::from_array([1400., 800.]);
pub static GAME_AREA_SIZE_U: Vec2 = Vec2::from_array([4000., 4000.]);
pub static MENU_AREA_SIZE_PX: Vec2 = Vec2::from_array([500., WINDOW_SIZE_PX.y]);

pub static MIN_DISTANCE: f32 = 10.; // Distance at which particles start to repel each other regardless of their attraction
pub static MAX_DISTNACE: f32 = 20.; // Distance at which particles stop having an effect on each other
pub static REPEL_CONSTANT: f32 = 0.5;
pub static ATTRACTION_CONSTANT: f32 = 0.005;
pub static NUM_PARTICLES: usize = 10000;
pub static PARTICLE_RADIUS: f32 = 3.;
pub static PARTICLE_FRICTION: f32 = 0.1;
// pub static COLORS: [Color; 7] = [RED, ORANGE, YELLOW, WHITE, GREEN, BLUE, VIOLET];
pub static COLORS: [Color; 3] = [RED, GREEN, BLUE];

pub static CAMERA_DRAG_SPEED: f32 = 750.;
pub static CAMERA_ZOOM_SPEED: f32 = 0.1;

pub static CHANGE_TYPE_ATTRACTION_SPEED: f32 = 0.02;
pub static MENU_BACKGORUND_COLOR: Color = DARKGRAY;

pub static QUADTREE_CAPACITY: usize = 20;
