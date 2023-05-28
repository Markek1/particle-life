use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

use crate::config::*;
use crate::helpers::ClickType;
use crate::particle::ParticleType;

#[derive(Clone, Copy)]
pub struct Area {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Area {
    pub fn new(pos: [f32; 2], size: [f32; 2]) -> Self {
        Self {
            pos: Vec2::new(pos[0] as f32, pos[1] as f32),
            size: Vec2::new(size[0] as f32, size[1] as f32),
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.pos.x
            && point.x <= self.pos.x + self.size.x
            && point.y >= self.pos.y
            && point.y <= self.pos.y + self.size.y
    }
}

enum ButtonType {
    Rect,
    Circle,
}

struct Button {
    area: Area,
    label: String,
    color: Color,
    btn_type: ButtonType,
    border_width: f32,
}

impl Button {
    pub fn new(area: Area, label: String, color: Color, btn_type: ButtonType) -> Self {
        let mut area = area;
        area.size -= 2.;
        Self {
            area: area,
            label,
            color,
            btn_type,
            border_width: 1.,
        }
    }

    pub fn draw(&self) {
        match self.btn_type {
            ButtonType::Rect => {
                draw_rectangle(
                    self.area.pos.x,
                    self.area.pos.y,
                    self.area.size.x,
                    self.area.size.y,
                    self.color,
                );

                draw_rectangle_lines(
                    self.area.pos.x,
                    self.area.pos.y,
                    self.area.size.x,
                    self.area.size.y,
                    self.border_width,
                    BLACK,
                )
            }
            ButtonType::Circle => {
                draw_circle(
                    self.area.pos.x + self.area.size.x / 2.0,
                    self.area.pos.y + self.area.size.y / 2.0,
                    self.area.size.x / 2.0,
                    self.color,
                );
                draw_circle_lines(
                    self.area.pos.x + self.area.size.x / 2.0,
                    self.area.pos.y + self.area.size.y / 2.0,
                    self.area.size.x / 2.0,
                    self.border_width,
                    BLACK,
                );
            }
        }

        draw_text(
            &self.label,
            self.area.pos.x + self.area.size.x / 2.0
                - measure_text(&self.label, None, 20, 1.0).width / 2.0,
            self.area.pos.y + self.area.size.y / 2.0,
            20.0,
            BLACK,
        );
    }
}

struct ButtonGrid {
    area: Area,
    buttons: Vec<Button>,
    row_buttons: Vec<Button>,
    column_buttons: Vec<Button>,
    rows: usize,
    cols: usize,
}

impl ButtonGrid {
    pub fn new(area: Area, rows: usize, cols: usize, types: &Vec<ParticleType>) -> Self {
        let button_size = area.size.x / (cols + 1) as f32;

        let mut row_buttons = Vec::new();
        for i in 1..(rows + 1) {
            let button = Button::new(
                Area::new(
                    [area.pos.x, area.pos.y + button_size * i as f32],
                    [button_size, button_size],
                ),
                format!(""),
                WHITE,
                ButtonType::Circle,
            );
            row_buttons.push(button);
        }

        let mut column_buttons = Vec::new();
        for i in 1..(cols + 1) {
            let button = Button::new(
                Area::new(
                    [area.pos.x + button_size * i as f32, area.pos.y],
                    [button_size, button_size],
                ),
                format!(""),
                WHITE,
                ButtonType::Circle,
            );
            column_buttons.push(button);
        }

        let mut buttons = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                let button = Button::new(
                    Area::new(
                        [
                            area.pos.x + button_size + button_size * j as f32,
                            area.pos.y + button_size + button_size * i as f32,
                        ],
                        [button_size, button_size],
                    ),
                    format!(""),
                    WHITE,
                    ButtonType::Rect,
                );
                buttons.push(button);
            }
        }

        let mut return_val = Self {
            area,
            buttons,
            row_buttons,
            column_buttons,
            rows,
            cols,
        };

        for i in 0..rows {
            for j in 0..cols {
                return_val.update_grid_button(i, j, types);
            }
        }

        return_val
    }

    pub fn click(&mut self, point: Vec2, types: &mut Vec<ParticleType>, click_type: ClickType) {
        // Ignore row and column buttons and if click was on a grid button, increase ParticleType attraction index
        for row in 0..self.rows {
            for col in 0..self.cols {
                let button = &mut self.buttons[row * self.cols + col];
                if button.area.contains(point) {
                    let attr = &mut types[row].attraction[col];
                    match click_type {
                        ClickType::Left => {
                            *attr = f32::min(*attr + CHANGE_TYPE_ATTRACTION_SPEED, 1.0);
                        }
                        ClickType::Right => {
                            *attr = f32::max(*attr - CHANGE_TYPE_ATTRACTION_SPEED, -1.0);
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&mut self, types: &Vec<ParticleType>) {
        draw_rectangle(
            self.area.pos.x,
            self.area.pos.y,
            self.area.size.x,
            self.area.size.y,
            MENU_BACKGORUND_COLOR,
        );

        for button in &self.row_buttons {
            button.draw();
        }

        for button in &self.column_buttons {
            button.draw();
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.update_grid_button(i, j, types);
            }
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.buttons[i * self.cols + j].draw();
            }
        }
    }

    fn update_grid_button(&mut self, row: usize, col: usize, types: &Vec<ParticleType>) {
        let button = &mut self.buttons[row * self.cols + col];

        let attr = types[row].attraction[col];
        button.label = format!("{:.2}", attr);

        // Gradient that goes through white between green and red
        button.color = if attr > 0.0 {
            Color::new(1. - attr, 1., 1. - attr, 1.)
        } else {
            Color::new(1., 1. + attr, 1. + attr, 1.)
        };
    }
}

pub struct Menu {
    pub area: Area,
    attraction_grid: ButtonGrid,
}

impl Menu {
    pub fn new(types: &Vec<ParticleType>) -> Self {
        let area = Area::new([0.0, 0.0], [MENU_AREA_SIZE_PX.x, MENU_AREA_SIZE_PX.y]);

        let grid_x_size = area.size.x / 1.1;
        let grid_area = Area::new(
            [
                area.pos.x + area.size.x / 2.0 - grid_x_size / 2.0,
                area.pos.y + area.size.x / 2.0 - grid_x_size / 2.0,
            ],
            [grid_x_size, grid_x_size],
        );

        let mut attraction_grid = ButtonGrid::new(grid_area, COLORS.len(), COLORS.len(), &types);

        for i in 0..COLORS.len() {
            attraction_grid.row_buttons[i].color = COLORS[i];
        }

        for i in 0..COLORS.len() {
            attraction_grid.column_buttons[i].color = COLORS[i];
        }

        Menu {
            area,
            attraction_grid,
        }
    }

    pub fn click(&mut self, point: Vec2, types: &mut Vec<ParticleType>, click_type: ClickType) {
        self.attraction_grid.click(point, types, click_type);
    }

    // Draw so that it stays in the same place on the screen
    pub fn draw(&mut self, types: &Vec<ParticleType>) {
        draw_rectangle(
            self.area.pos.x,
            self.area.pos.y,
            self.area.size.x,
            screen_height(),
            MENU_BACKGORUND_COLOR,
        );

        use macroquad::ui;

        root_ui().move_window(6, Vec2::new(200., 200.));

        let attraction_grid_bottom =
            self.attraction_grid.area.pos.y + self.attraction_grid.area.size.y;

        let area = self.area;
        let grid_x_size = area.size.x / 1.1;
        root_ui().window(
            hash!(),
            vec2(
                area.pos.x + area.size.x / 2.0 - grid_x_size / 2.0,
                attraction_grid_bottom + area.size.x / 2.0 - grid_x_size / 2.0,
            ),
            vec2(grid_x_size, 200.),
            |ui| {
                ui::widgets::Slider::new(hash!(), 0.01..5.)
                    .label("Repel")
                    .ui(ui, unsafe { &mut REPEL_CONSTANT });
                ui::widgets::Slider::new(hash!(), 0.001..0.05)
                    .label("Attract")
                    .ui(ui, unsafe { &mut ATTRACT_CONSTANT });
                ui::widgets::Slider::new(hash!(), 0.0..1.)
                    .label("Friction")
                    .ui(ui, unsafe { &mut PARTICLE_FRICTION });
            },
        );

        self.attraction_grid.draw(types);
    }
}
