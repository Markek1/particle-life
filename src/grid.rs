use macroquad::prelude::*;

pub struct Cell {
    rect: Rect,
    pub particles: Vec<usize>,
}

pub struct Grid {
    pub cells: Vec<Cell>,
    pub shape: (usize, usize),
    cell_size: f32,
}

impl Grid {
    pub fn new(game_area_size: Vec2, max_effect_range: f32) -> Self {
        let cell_size = max_effect_range;
        let shape = (
            (game_area_size.x / cell_size).ceil() as usize,
            (game_area_size.y / cell_size).ceil() as usize,
        );
        let mut cells = Vec::new();
        for y in 0..shape.1 as usize {
            for x in 0..shape.0 as usize {
                cells.push(Cell {
                    rect: Rect::new(
                        x as f32 * cell_size,
                        y as f32 * cell_size,
                        cell_size,
                        cell_size,
                    ),
                    particles: Vec::new(),
                });
            }
        }

        Self {
            cells,
            shape,
            cell_size,
        }
    }

    pub fn cell_pos_from_pos(&self, pos: Vec2) -> (usize, usize) {
        let mut x = (pos.x / self.cell_size).floor() as usize;
        x %= self.shape.0 as usize;

        let mut y = (pos.y / self.cell_size).floor() as usize;
        y %= self.shape.1 as usize;

        (x, y)
    }

    pub fn insert(&mut self, index: usize, pos: Vec2) {
        let (x, y) = self.cell_pos_from_pos(pos);
        self.cells[y * self.shape.0 + x].particles.push(index);
    }

    pub fn query_single(&self, pos: Vec2) -> &Vec<usize> {
        let (x, y) = self.cell_pos_from_pos(pos);
        let cell = &self.cells[y * self.shape.0 + x];
        &cell.particles
    }

    // Querry the cell that the position is in + the 8 surrounding cells
    pub fn query_surroundings(&self, pos: Vec2) -> Vec<usize> {
        let mut particles = Vec::new();
        let (x, y) = self.cell_pos_from_pos(pos);
        let x = x as isize;
        let y = y as isize;

        for i in (y.overflowing_sub(1).0)..=(y.overflowing_add(1).0) {
            for j in (x.overflowing_sub(1).0)..=(x.overflowing_add(1).0) {
                // println!("i: {}, j: {}", i, j);
                // println!("Self shape: {} {}", self.shape.0, self.shape.1);
                // println!(
                //     "Mod: {} {}",
                //     (i % self.shape.0 as isize),
                //     (j % self.shape.1 as isize)
                // );
                let i = i.rem_euclid(self.shape.0 as isize) as usize;
                let j = j.rem_euclid(self.shape.1 as isize) as usize;

                let cell = &self.cells[i * self.shape.0 as usize + j];
                for particle in &cell.particles {
                    particles.push(*particle);
                }
            }
        }
        particles
    }

    pub fn draw(&self) {
        for cell in &self.cells {
            draw_rectangle_lines(
                cell.rect.x,
                cell.rect.y,
                cell.rect.w,
                cell.rect.h,
                5.,
                WHITE,
            );
        }
    }
}
