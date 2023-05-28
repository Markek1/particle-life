use macroquad::prelude::*;

use crate::config::*;
use crate::quadtree::Quadtree;
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
    quadtree: Quadtree,
}

impl Particles {
    pub fn new(game_area_size: Vec2) -> Self {
        Self {
            particles: Vec::new(),
            num_particles: 0,
            quadtree: Quadtree::new(
                QUADTREE_CAPACITY,
                Rect::new(0., 0., game_area_size.x, game_area_size.y),
            ),
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.quadtree.insert(self.num_particles, particle.pos);
        self.particles.push(particle);
        self.num_particles += 1;
    }

    pub fn update(&mut self, types: &Vec<ParticleType>) {
        self.quadtree = Quadtree::new(
            QUADTREE_CAPACITY,
            Rect::new(0., 0., GAME_AREA_SIZE_U.x, GAME_AREA_SIZE_U.y),
        );
        for i in 0..self.num_particles {
            self.quadtree.insert(i, self.particles[i].pos);
        }

        for i in 0..self.num_particles {
            let p1 = &self.particles[i];
            for j in self.quadtree.query(Circle {
                x: p1.pos.x,
                y: p1.pos.y,
                r: MAX_DISTNACE.max(MIN_DISTANCE),
            }) {
                if i == j {
                    continue;
                }
                let typeid2 = self.particles[j].type_id;
                let type1 = &types[self.particles[i].type_id];

                let mut d = self.particles[j].pos - self.particles[i].pos;

                if d.x.abs() > GAME_AREA_SIZE_U.x / 2. {
                    d.x = -d.x.signum() * (GAME_AREA_SIZE_U.x - d.x.signum() * d.x);
                }
                if d.y.abs() > GAME_AREA_SIZE_U.y / 2. {
                    d.y = -d.y.signum() * (GAME_AREA_SIZE_U.y - d.y.signum() * d.y);
                }
                let distance = d.length();
                d = d.normalize();

                if distance < MIN_DISTANCE {
                    self.particles[i].vel -=
                        REPEL_CONSTANT * d * (MIN_DISTANCE - distance) / MIN_DISTANCE;
                } else if distance < MAX_DISTNACE {
                    let num = (distance - (MAX_DISTNACE + MIN_DISTANCE) / 2.).abs();
                    let den = MAX_DISTNACE - MIN_DISTANCE;
                    self.particles[i].vel +=
                        ATTRACTION_CONSTANT * d * type1.attraction[typeid2] * (1. - num / den);
                }
            }
        }

        for i in 0..self.num_particles {
            let vel = self.particles[i].vel;

            // Wrap around
            self.particles[i].pos += vel;
            if self.particles[i].pos.x < 0. {
                self.particles[i].pos.x = GAME_AREA_SIZE_U.x;
            } else if self.particles[i].pos.x > GAME_AREA_SIZE_U.x {
                self.particles[i].pos.x = 0.;
            }
            if self.particles[i].pos.y < 0. {
                self.particles[i].pos.y = GAME_AREA_SIZE_U.y;
            } else if self.particles[i].pos.y > GAME_AREA_SIZE_U.y {
                self.particles[i].pos.y = 0.;
            }

            self.particles[i].vel *= 1. - PARTICLE_FRICTION;
        }
    }

    pub fn draw(&self, types: &Vec<ParticleType>) {
        // self.quadtree.draw();

        for i in 0..self.num_particles {
            let type1 = &types[self.particles[i].type_id];
            draw_circle(
                self.particles[i].pos.x,
                self.particles[i].pos.y,
                PARTICLE_RADIUS,
                type1.color,
            );
        }

        // let range = Circle::new(mouse_position().0, mouse_position().1, 200.);
        // let found = self.quadtree.query(range);
        // for i in 0..found.len() {
        //     draw_circle(
        //         self.particles[found[i]].pos.x,
        //         self.particles[found[i]].pos.y,
        //         PARTICLE_RADIUS,
        //         WHITE,
        //     );
        // }
    }
}
