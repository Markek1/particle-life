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
        let mut cell_size = max_effect_range;
        let shape = (
            (game_area_size.x / cell_size).floor() as usize,
            (game_area_size.y / cell_size).floor() as usize,
        );
        cell_size += (game_area_size.x % cell_size) / shape.0 as f32;
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

    #[allow(unused)]
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
