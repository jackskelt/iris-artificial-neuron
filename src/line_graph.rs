use std::ops::Add;

use raylib::prelude::*;

pub struct LineGraph {
    pub data: Vec<f64>,
    pub color: Color,
    pub thickness: f32,
    pub max_data_points: usize,
    pub pos: Vector2,
    pub width: f32,
    pub height: f32,
}

pub const Y_MARKS: usize = 2;
pub const FONT_SIZE: i32 = 15;

impl LineGraph {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            data: Vec::new(),
            color: Color::BLACK,
            thickness: 3.0,
            max_data_points: 1000,
            pos: Vector2 { x, y },
            width,
            height,
        }
    }

    // This will draw the frame of the graph. It's just the left and bottom border.
    fn draw_frame(&self, d: &mut RaylibDrawHandle) {
        let intersection_point = self.pos.clone().add(Vector2::new(0.0, self.height));
        d.draw_line_ex(self.pos, intersection_point, 1.0, Color::BLACK);
        d.draw_line_ex(
            intersection_point,
            intersection_point
                .clone()
                .add(Vector2::new(self.width, 0.0)),
            1.0,
            Color::BLACK,
        );
    }

    // Draw the guide values with their numbers on the left side of the graph frame. The guide values should be proportional to the data values, always fixed values inside the limits of the graph.
    fn draw_guide_values(&self, d: &mut RaylibDrawHandle) {
        let max_data = self.data.iter().fold(f64::MIN, |a, &b| a.max(b));
        let min_data = self.data.iter().fold(f64::MAX, |a, &b| a.min(b));

        let data_range = max_data - min_data;

        let y_step = self.height / Y_MARKS as f32;

        for i in 0..=Y_MARKS {
            let y = self.pos.y + self.height - i as f32 * y_step;
            let value = format!("{:.2}", min_data + i as f64 / Y_MARKS as f64 * data_range);

            let size = d.measure_text(&value, FONT_SIZE);

            d.draw_text(
                &value,
                (self.pos.x as i32) - size - 5,
                y as i32 - FONT_SIZE / 2,
                FONT_SIZE,
                self.color,
            );
        }
    }

    // Draw the guive values with their numbers on the left side of the graph
    // fn draw_guide_values(&self, d: &mut RaylibDrawHandle) {
    //     let max_data = self.data.iter().fold(f32::MIN, |a, &b| a.max(b));
    //     let min_data = self.data.iter().fold(f32::MAX, |a, &b| a.min(b));

    //     let data_range = max_data - min_data;

    //     let y_step = self.height / Y_MARKS as f32;

    //     for i in 0..=Y_MARKS {
    //         let y = self.y + self.height - i as f32 * y_step;
    //         let value = min_data + i as f32 / Y_MARKS as f32 * data_range;

    //         d.draw_text(&format!("{:.2}", value), self.x as i32, y as i32, 10, self.color);
    //     }
    // }

    // Make a draw function that is proportional to the values of data, to always represent values inside the limits of the graph
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.draw_frame(d);

        self.draw_guide_values(d);

        if self.data.len() > 1 {
            let mut x = self.pos.x;
            let y = self.pos.y + self.height;

            let max_data = self.data.iter().fold(f64::MIN, |a, &b| a.max(b));
            let min_data = self.data.iter().fold(f64::MAX, |a, &b| a.min(b));

            let data_range = (max_data - min_data) as f32;

            let x_step = self.width as f64 / self.max_data_points as f64;
            let y_step = (self.height / data_range) as f64;

            let mut last_data = self.data[0];

            let current_data = self.data.last().unwrap();
            d.draw_text(
                &format!("{:.2}", current_data),
                (x + self.width) as i32 + 5,
                (y as f64 - (current_data - min_data) * y_step) as i32,
                FONT_SIZE,
                self.color,
            );

            for data in self.data.iter().skip(1) {
                let x1 = x;
                let y1 = y - ((last_data - min_data) * y_step) as f32;
                let x2 = x + x_step as f32;
                let y2 = y - ((data - min_data) * y_step) as f32;

                d.draw_line_ex(
                    Vector2::new(x1, y1),
                    Vector2::new(x2, y2),
                    self.thickness,
                    self.color,
                );

                x = x2;
                last_data = *data;
            }
        }
    }

    pub fn add_data(&mut self, data: f64) {
        self.data.push(data);
        if self.data.len() > self.max_data_points {
            self.data.remove(0);
        }
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}
