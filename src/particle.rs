use core_affinity;
use macroquad::prelude::*;
use std::thread;

use crate::config::*;
use crate::grid::{Cell, Grid};
#[derive(Clone)]
pub struct ParticleType {
    pub color: Color,
    pub attraction: Vec<f32>,
}

impl ParticleType {
    pub fn new(color: Color, attraction: Vec<f32>) -> Self {
        Self { color, attraction }
    }
}

pub struct Particle {
    pos: Vec2,
    vel: Vec2,
    type_id: usize, // index of the type in the type manager
}

impl Particle {
    pub fn new(pos: [f32; 2], vel: [f32; 2], type_id: usize) -> Self {
        Self {
            pos: Vec2::new(pos[0] as f32, pos[1] as f32),
            vel: Vec2::new(vel[0] as f32, vel[1] as f32),
            type_id,
        }
    }
}

pub struct Particles {
    particles: Vec<Particle>,
    num_particles: usize,
    grid: Grid,
}

impl Particles {
    pub fn new(game_area_size: Vec2) -> Self {
        Self {
            particles: Vec::new(),
            num_particles: 0,
            grid: Grid::new(game_area_size, MAX_DISTNACE.max(MIN_DISTANCE)),
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.grid.insert(self.num_particles, particle.pos);
        self.particles.push(particle);
        self.num_particles += 1;
    }

    pub fn update_cell(&mut self, cell: &Cell, types: &Vec<ParticleType>) {
        let cell_x = cell.pos.0 as isize;
        let cell_y = cell.pos.1 as isize;

        // Basically do a convolution with a 3x3 kernel over the cells and update each particle
        // in the central cell based on every particle (except itself) in every surrounding
        // cell (including its own cell).
        // This is done to avoid having to check every particle against every other particle
        // in the game.
        // The cells wrap around the edges of the game area.
        let cell1 = &self.grid.cells[cell_y as usize * self.grid.shape.0 + cell_x as usize];
        for cell_i in (cell_y.overflowing_sub(1).0)..=(cell_y.overflowing_add(1).0) {
            for cell_j in (cell_x.overflowing_sub(1).0)..=(cell_x.overflowing_add(1).0) {
                let cell_i = cell_i.rem_euclid(self.grid.shape.1 as isize) as usize;
                let cell_j = cell_j.rem_euclid(self.grid.shape.0 as isize) as usize;

                for pi in 0..cell1.particles.len() {
                    let cell2 = &self.grid.cells[cell_i * self.grid.shape.0 + cell_j];
                    for pj in 0..cell2.particles.len() {
                        let i = cell1.particles[pi];
                        let j = cell2.particles[pj];
                        if i == j {
                            continue;
                        }
                        let type1 = &types[self.particles[i].type_id];
                        let typeid2 = self.particles[j].type_id;

                        let mut d = self.particles[j].pos - self.particles[i].pos;

                        if d.x.abs() > GAME_AREA_SIZE_U.x / 2. {
                            d.x = -d.x.signum() * (GAME_AREA_SIZE_U.x - d.x.abs());
                        }
                        if d.y.abs() > GAME_AREA_SIZE_U.y / 2. {
                            d.y = -d.y.signum() * (GAME_AREA_SIZE_U.y - d.y.abs());
                        }

                        let distance = d.length();
                        d = d / distance;

                        // Unsafe because of using static variables that might be changed
                        unsafe {
                            if distance < MIN_DISTANCE {
                                self.particles[i].vel -=
                                    REPEL_CONSTANT * d * (MIN_DISTANCE - distance) / MIN_DISTANCE;
                            } else if distance < MAX_DISTNACE {
                                let num = (distance - (MAX_DISTNACE + MIN_DISTANCE) / 2.).abs();
                                let den = MAX_DISTNACE - MIN_DISTANCE;
                                self.particles[i].vel += ATTRACT_CONSTANT
                                    * d
                                    * type1.attraction[typeid2]
                                    * (1. - num / den);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self, types: &Vec<ParticleType>) {
        self.grid = Grid::new(GAME_AREA_SIZE_U, MAX_DISTNACE.max(MIN_DISTANCE));
        for i in 0..self.num_particles {
            self.grid.insert(i, self.particles[i].pos);
        }

        let self_ptr = self as *const _ as usize;

        let num_cpus = num_cpus::get().min(MAX_CORES);
        let core_ids = core_affinity::get_core_ids().expect("Could not get core IDs");
        let cells_per_cpu = (self.grid.cells.len() as f32 / num_cpus as f32).ceil() as usize;
        let cell_chunks = self.grid.cells.chunks_mut(cells_per_cpu);

        thread::scope(|s| {
            for (chunk, core_id) in cell_chunks.zip(core_ids) {
                s.spawn(move || {
                    core_affinity::set_for_current(core_id);
                    for cell in chunk {
                        unsafe {
                            // Epic way to avoid refactoring the code.
                            // It should be safe because update_cells only needs
                            // the position of other particles and only
                            // changes velocities, so doing it in parallel should be okay.
                            // The positions are updated based on their velocities later
                            let slf = &mut *(self_ptr as *mut Self);

                            slf.update_cell(&cell, types);
                        }
                    }
                });
            }
        });

        for i in 0..self.num_particles {
            let vel = self.particles[i].vel;

            self.particles[i].pos += vel;
            // Wrap around
            if self.particles[i].pos.x < 0. {
                self.particles[i].pos.x = GAME_AREA_SIZE_U.x;
            } else if self.particles[i].pos.x >= GAME_AREA_SIZE_U.x {
                self.particles[i].pos.x = 0.;
            }
            if self.particles[i].pos.y < 0. {
                self.particles[i].pos.y = GAME_AREA_SIZE_U.y;
            } else if self.particles[i].pos.y >= GAME_AREA_SIZE_U.y {
                self.particles[i].pos.y = 0.;
            }

            unsafe { self.particles[i].vel *= 1. - PARTICLE_FRICTION };
        }
    }

    pub fn draw(&self, types: &Vec<ParticleType>, camera: &Camera2D) {
        // self.grid.draw();

        for i in 0..self.num_particles {
            let particle = &self.particles[i];

            // Do not draw a particle if it's not visible on camera (HUGE PERFORMANCE BOOST)
            let pos_on_screen = camera.world_to_screen(particle.pos);
            if pos_on_screen.x < 0.
                || pos_on_screen.x > screen_width()
                || pos_on_screen.y < 0.
                || pos_on_screen.y > screen_height()
            {
                continue;
            }

            let type1 = &types[particle.type_id];
            draw_circle(particle.pos.x, particle.pos.y, PARTICLE_RADIUS, type1.color);
        }
    }
}
