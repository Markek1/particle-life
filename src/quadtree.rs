use macroquad::prelude::*;

/// Contains index to a vector of objects that might be used by some higher data structure
struct Point {
    index: usize,
    pos: Vec2,
}

pub struct Quadtree {
    capacity: usize,
    boundary: Rect,
    points: Vec<Point>,
    divided: bool,
    children: Option<Vec<Quadtree>>,
}

impl Quadtree {
    pub fn new(capacity: usize, boundary: Rect) -> Self {
        Self {
            capacity,
            boundary,
            points: Vec::new(),
            divided: false,
            children: None,
        }
    }

    pub fn divide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w;
        let h = self.boundary.h;

        let nw = Rect::new(x, y, w / 2., h / 2.);
        let ne = Rect::new(x + w / 2., y, w / 2., h / 2.);
        let sw = Rect::new(x, y + h / 2., w / 2., h / 2.);
        let se = Rect::new(x + w / 2., y + h / 2., w / 2., h / 2.);

        self.children = Some(vec![
            Quadtree::new(self.capacity, nw),
            Quadtree::new(self.capacity, ne),
            Quadtree::new(self.capacity, sw),
            Quadtree::new(self.capacity, se),
        ]);

        self.divided = true;
    }

    // Create children when inserting into a full node
    pub fn insert(&mut self, index: usize, pos: Vec2) -> bool {
        if !self.boundary.contains(pos) {
            return false;
        }

        if self.points.len() < self.capacity {
            self.points.push(Point { index, pos });
            return true;
        }

        if !self.divided {
            self.divide();
        }

        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.insert(index, pos) {
                    return true;
                }
            }
        }

        false
    }

    fn query_rec(&self, range: Circle, indices: &mut Vec<usize>) {
        if !range.overlaps_rect(&self.boundary) {
            return;
        }

        for point in &self.points {
            if range.contains(&point.pos) {
                indices.push(point.index);
            }
        }
        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query_rec(range, indices);
            }
        }
    }

    pub fn query(&self, range: Circle) -> Vec<usize> {
        let mut indices = Vec::new();

        self.query_rec(range, &mut indices);

        indices
    }

    pub fn draw(&self) {
        draw_rectangle_lines(
            self.boundary.x,
            self.boundary.y,
            self.boundary.w,
            self.boundary.h,
            4.,
            WHITE,
        );

        if let Some(children) = &self.children {
            for child in children.iter() {
                child.draw();
            }
        }
    }
}
