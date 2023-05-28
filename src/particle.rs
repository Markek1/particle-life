use macroquad::prelude::*;

use crate::config::*;

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
}

impl Particles {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            num_particles: 0,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
        self.num_particles += 1;
    }

    pub fn update(&mut self, types: &Vec<ParticleType>) {
        for i in 0..self.num_particles {
            for j in 0..self.num_particles {
                if i != j {
                    let typeid2 = self.particles[j].type_id;
                    let type1 = &types[self.particles[i].type_id];

                    let mut force = self.particles[j].pos - self.particles[i].pos;
                    let distance = force.length();

                    force = force.normalize();

                    if distance < MIN_DISTANCE {
                        self.particles[i].vel -=
                            REPEL_CONSTANT * force * (MIN_DISTANCE - distance) / MIN_DISTANCE;
                    } else if distance < MAX_DISTNACE {
                        let num = (distance - (MAX_DISTNACE + MIN_DISTANCE) / 2.).abs();
                        let den = MAX_DISTNACE - MIN_DISTANCE;
                        self.particles[i].vel += ATTRACTION_CONSTANT
                            * force
                            * type1.attraction[typeid2]
                            * (1. - num / den);
                    }
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
        for i in 0..self.num_particles {
            let type1 = &types[self.particles[i].type_id];
            draw_circle(
                self.particles[i].pos.x,
                self.particles[i].pos.y,
                PARTICLE_RADIUS,
                type1.color,
            );
        }
    }
}
