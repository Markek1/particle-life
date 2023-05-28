use macroquad::prelude::*;

use crate::config::*;
use crate::helpers::ClickType;
use crate::particle::{Particle, ParticleType, Particles};
use crate::ui::Menu;

pub struct Game {
    particles: Particles,
    types: Vec<ParticleType>,
    paused: bool,
    pub camera_zoom: f32,
    prev_mouse_lclick_pos: Option<Vec2>,
    menu: Menu,
    camera: Camera2D,
}

impl Game {
    pub fn new() -> Self {
        let types = Self::initialize_types(None);
        let particles = Self::initialize_particles(&types);
        let menu = Menu::new(&types);

        Game {
            particles,
            types,
            paused: false,
            camera_zoom: 2. / screen_width(),
            prev_mouse_lclick_pos: None,
            menu,
            camera: Camera2D::from_display_rect(Rect::new(
                0.,
                0.,
                WINDOW_SIZE_PX.x,
                WINDOW_SIZE_PX.y,
            )),
        }
    }

    fn initialize_types(value: Option<f32>) -> Vec<ParticleType> {
        let mut types = Vec::new();

        for color in &COLORS {
            let mut attract_vec = Vec::new();
            for _ in 0..COLORS.len() {
                match value {
                    Some(value) => attract_vec.push(value),
                    None => attract_vec.push(rand::gen_range(-1., 1.)),
                };
            }

            types.push(ParticleType::new(*color, attract_vec));
        }

        types
    }

    fn initialize_particles(types: &Vec<ParticleType>) -> Particles {
        let mut particles = Particles::new();

        for _ in 0..NUM_PARTICLES {
            particles.add_particle(Particle::new(
                [
                    rand::gen_range(0., GAME_AREA_SIZE_U.x),
                    rand::gen_range(0., GAME_AREA_SIZE_U.y),
                ],
                [0., 0.],
                rand::gen_range(0, types.len()),
            ));
        }

        particles
    }

    pub fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.paused = !self.paused;
        }

        if is_key_pressed(KeyCode::R) {
            self.particles = Self::initialize_particles(&self.types);
        }
        if is_key_pressed(KeyCode::C) {
            self.types = Self::initialize_types(Some(0.));
        }
        if is_key_pressed(KeyCode::N) {
            self.types = Self::initialize_types(None);
            self.particles = Self::initialize_particles(&self.types);
        }

        let mouse_pos = mouse_position();
        let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);
        if is_mouse_button_down(MouseButton::Left) {
            if !self.menu.area.contains(mouse_pos) {
                match self.prev_mouse_lclick_pos {
                    None => {
                        self.prev_mouse_lclick_pos = Some(mouse_pos);
                    }
                    Some(prev_pos) => {
                        self.camera.target -= Vec2::new(1., -1.) * (mouse_pos - prev_pos)
                            / (self.camera_zoom * CAMERA_DRAG_SPEED);

                        self.prev_mouse_lclick_pos = Some(mouse_pos);
                    }
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if self.menu.area.contains(mouse_pos) {
                self.menu.click(mouse_pos, &mut self.types, ClickType::Left);
            }
        }
        if is_mouse_button_down(MouseButton::Right) {
            if self.menu.area.contains(mouse_pos) {
                self.menu
                    .click(mouse_pos, &mut self.types, ClickType::Right);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.prev_mouse_lclick_pos = None;
        }

        match mouse_wheel() {
            (_x, y) if y != 0.0 => {
                // // Normalize mouse wheel values is browser (chromium: 53, firefox: 3)
                // #[cfg(target_arch = "wasm32")]
                let y = if y < 0.0 {
                    -1.0
                } else if y > 0.0 {
                    1.0
                } else {
                    0.0
                };

                let factor = (1. + CAMERA_ZOOM_SPEED).powf(y);
                self.camera_zoom *= factor;

                // Zoom in to cursor position
                let view_size_u = 2.
                    / Vec2::new(
                        self.camera_zoom,
                        self.camera_zoom * screen_width() / screen_height(),
                    );

                let mouse_pos_p =
                    Vec2::new(mouse_position().0, screen_height() - mouse_position().1);
                let screen_size_p = Vec2::new(screen_width(), screen_height());
                let zoom_level = screen_size_p / view_size_u;
                let diff_p = mouse_pos_p - screen_size_p / 2.;
                let diff_u = diff_p / zoom_level;

                self.camera.target -= diff_u * (1. - factor);
            }
            _ => (),
        }
    }

    pub fn update(&mut self) {
        if !self.paused {
            self.particles.update(&self.types);
        }
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);

        self.particles.draw(&self.types);

        set_default_camera(); // For drawing the menu

        self.menu.draw(&self.types);

        set_camera(&Camera2D {
            target: self.camera.target,
            zoom: self.camera_zoom * Vec2::new(1., screen_width() / screen_height()),
            ..Default::default()
        });
    }
}
