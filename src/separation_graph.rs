use std::ops::Add;

// This will be a graph that will have two groups of points, each one with its color, and the line that separates them. The line will be drawn with the least squares method.
use raylib::prelude::*;

type DataVector = Vec<(f32, f32, Color)>;

pub struct SeparationGraph {
    pub pos: Vector2,
    pub width: f32,
    pub height: f32,
    pub data: DataVector,
    pub line_color: Color,
    pub max_point: Vector2,
    pub min_point: Vector2,
    pub decision_line: Option<(f32, f32)>,
    pub x_axis: String,
    pub y_axis: String,
}

pub const Y_MARKS: usize = 4;
pub const X_MARKS: usize = Y_MARKS;
pub const FONT_SIZE: i32 = 15;
pub const DOT_RADIUS: f32 = 5.0;
pub const PADDING: f32 = 0.3;

impl SeparationGraph {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        x_axis: String,
        y_axis: String,
        data: DataVector,
    ) -> Self {
        let max_x = data.iter().fold(f32::MIN, |a, &(x, _, _)| a.max(x));
        let max_y = data.iter().fold(f32::MIN, |a, &(_, y, _)| a.max(y));
        let min_x = data.iter().fold(f32::MAX, |a, &(x, _, _)| a.min(x));
        let min_y = data.iter().fold(f32::MAX, |a, &(_, y, _)| a.min(y));

        Self {
            pos: Vector2::new(x, y),
            width,
            height,
            data,
            line_color: Color::BLUE,
            max_point: Vector2::new(max_x, max_y),
            min_point: Vector2::new(min_x, min_y),
            x_axis,
            decision_line: None,
            y_axis,
        }
    }

    // Draw the frame of the graph. It's just the left and bottom border.
    // Write the name of the axis on the end of each line
    // Create and arrow for the frame
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

        let x_axis_text_size = d.measure_text(&self.x_axis, FONT_SIZE) as f32;
        let y_axis_text_size = d.measure_text(&self.y_axis, FONT_SIZE);

        d.draw_text(
            &self.x_axis,
            (self.pos.x + self.width - x_axis_text_size) as i32,
            (self.pos.y + self.height + 5.0) as i32 + FONT_SIZE,
            FONT_SIZE,
            Color::BLACK,
        );

        // Verify if the final text pos is negative to not overlap the frame
        let mut x_pos = (self.pos.x as i32) - y_axis_text_size - 5;

        if x_pos < 0 {
            x_pos += y_axis_text_size + 5;
        }

        d.draw_text(
            &self.y_axis,
            x_pos,
            self.pos.y as i32 - FONT_SIZE - 5,
            FONT_SIZE,
            Color::BLACK,
        );
    }

    // This function should calculate the points of the decision line and the max and min points of the graph and should consider the decision line.
    pub fn set_decision_line(&mut self, weights: &Vec<f64>, bias: f64) {
        let x1 = self.min_point.x as f64;
        let x2 = self.max_point.x as f64;

        let y1 = (-weights[0] * x1 - bias) / weights[1];
        let y2 = (-weights[0] * x2 - bias) / weights[1];

        self.decision_line = Some((y1 as f32, y2 as f32));

        let max_x = self.data.iter().fold(f32::MIN, |a, &(x, _, _)| a.max(x));
        let max_y = self.data.iter().fold(f32::MIN, |a, &(_, y, _)| a.max(y));
        let min_x = self.data.iter().fold(f32::MAX, |a, &(x, _, _)| a.min(x));
        let min_y = self.data.iter().fold(f32::MAX, |a, &(_, y, _)| a.min(y));

        let y1 = self.decision_line.unwrap().0;
        let y2 = self.decision_line.unwrap().1;

        let max_y = max_y.max(y1).max(y2);
        let min_y = min_y.min(y1).min(y2);

        self.max_point = Vector2::new(max_x, max_y);
        self.min_point = Vector2::new(min_x, min_y);
    }

    // Draw the guide values with their numbers on the left side of the graph frame. The guide values should be proportional to the data values and separator line, always fixed values inside the limits of the graph.
    // The guide should be in the y and x axis.
    // Draw some light lines to guide the values.
    fn draw_guide_values(&self, d: &mut RaylibDrawHandle) {
        let y_step = self.height / Y_MARKS as f32;
        let x_step = self.width / X_MARKS as f32;

        for i in 0..=Y_MARKS {
            let y = self.pos.y + self.height - i as f32 * y_step;
            let value = format!(
                "{:.2}",
                self.min_point.y - PADDING
                    + i as f32 / Y_MARKS as f32
                        * (self.max_point.y - self.min_point.y - 2.0 * -PADDING)
            );

            let size = d.measure_text(&value, FONT_SIZE);

            d.draw_text(
                &value,
                self.pos.x as i32 - size - 5,
                y as i32 - FONT_SIZE / 2,
                FONT_SIZE,
                Color::BLACK,
            );

            d.draw_line_ex(
                Vector2::new(self.pos.x, y),
                Vector2::new(self.pos.x + self.width, y),
                1.0,
                Color::new(0, 0, 0, 100),
            );
        }

        for i in 0..=X_MARKS {
            let x = self.pos.x + i as f32 * x_step;
            let value = format!(
                "{:.2}",
                self.min_point.x - PADDING
                    + i as f32 / X_MARKS as f32
                        * (self.max_point.x - self.min_point.x - 2.0 * -PADDING)
            );

            let size = d.measure_text(&value, FONT_SIZE);

            d.draw_text(
                &value,
                x as i32 - size / 2,
                (self.pos.y + self.height + 5.0) as i32,
                FONT_SIZE,
                Color::BLACK,
            );

            d.draw_line_ex(
                Vector2::new(x, self.pos.y),
                Vector2::new(x, self.pos.y + self.height),
                1.0,
                Color::new(200, 200, 200, 255),
            );
        }
    }

    // Draw the boundary line that separates the two groups of points based on the weights and bias. The line should be proportional to the graph.
    // The tecnique is logical regression, the line is the result of the least squares method.
    fn draw_separation_line(&self, d: &mut RaylibDrawHandle) {
        if let Some((y1, y2)) = self.decision_line {
            let x1 = self.min_point.x;
            let x2 = self.max_point.x;

            let x1 = self.get_x_proportional(x1);
            let x2 = self.get_x_proportional(x2);
            let y1 = self.get_y_proportional(y1);
            let y2 = self.get_y_proportional(y2);

            d.draw_line_ex(
                Vector2::new(x1, y1),
                Vector2::new(x2, y2),
                2.0,
                self.line_color,
            );
        }
    }

    // Draw the points of the graph. Each point will have its color. Should be proportional to graph
    fn draw_points(&self, d: &mut RaylibDrawHandle) {
        for (x, y, color) in &self.data {
            let x = self.get_x_proportional(*x);
            let y = self.get_y_proportional(*y);

            d.draw_circle_v(Vector2::new(x, y), DOT_RADIUS, *color);
        }
    }

    fn get_x_proportional(&self, x: f32) -> f32 {
        let range_x = self.max_point.x - self.min_point.x - 2.0 * -PADDING;
        let min_x = self.min_point.x - PADDING;

        self.pos.x + ((x - min_x) / range_x) * self.width
    }

    fn get_y_proportional(&self, y: f32) -> f32 {
        let range_y = self.max_point.y - self.min_point.y - 2.0 * -PADDING;
        let min_y = self.min_point.y - PADDING;

        self.pos.y + self.height - ((y - min_y) / range_y) * self.height
    }

    pub fn remove_decision_line(&mut self) {
        self.decision_line = None;
    }

    // Make a draw function that is proportional to the values of data, to always represent values inside the limits of the graph
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.draw_frame(d);

        self.draw_guide_values(d);

        self.draw_separation_line(d);

        self.draw_points(d);
    }
}
