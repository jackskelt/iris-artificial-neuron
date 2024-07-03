use rand::Rng;
use raylib::prelude::*;
use std::f64::consts::E;

#[derive(Debug)]
pub struct Activation {
    pub function: fn(f64) -> f64,
    pub derivative: fn(f64) -> f64,
}

pub const SIGMOID: Activation = Activation {
    function: |x| 1.0 / (1.0 + E.powf(-x)),
    derivative: |x| x * (1.0 - x),
};

#[derive(Debug)]
pub struct Neuron {
    pub inputs: usize,
    pub weights: Vec<f64>,
    pub biase: f64,
    pub data: Vec<f64>,
    pub activation: Activation,
    pub learning_rate: f64,
    pub loss: f64,
    pub amount: f64,
}

impl Neuron {
    pub fn new(inputs: usize, activation: Activation, learning_rate: f64) -> Neuron {
        let mut weights = Vec::new();

        for _ in 0..inputs {
            weights.push(rand::thread_rng().gen_range(0.0..1.0));
        }

        let biase = rand::thread_rng().gen_range(0.0..1.0);

        // let weights = vec![0.5, 0.5];
        // let biase = 0.5;
        Neuron {
            inputs,
            weights,
            biase,
            data: vec![0.0; inputs],
            activation,
            learning_rate,
            loss: 0.0,
            amount: 0.0,
        }
    }

    pub fn feed_forward(&mut self, inputs: &Vec<f64>) -> f64 {
        assert!(self.inputs == inputs.len(), "Invalid number of inputs");

        self.data = inputs.clone();

        let mut sum = self.biase;

        for i in 0..self.inputs {
            sum += inputs[i] * self.weights[i];
        }

        (self.activation.function)(sum)
    }

    pub fn back_propagate(&mut self, output: f64, target: f64) -> (f64, f64) {
        let error = target - output;
        let gradient = (self.activation.derivative)(output) * error * self.learning_rate;

        for i in 0..self.inputs {
            self.weights[i] += gradient * self.data[i];
        }

        self.biase += gradient;

        (error, gradient)
    }

    pub fn loss(&mut self, output: f64, target: f64) -> f64 {
        0.5 * (target - output).powi(2)
    }

    pub fn log_loss(&mut self, output: f64, target: f64) -> f64 {
        let loss = target * f64::ln(output) + (1.0 - target) * f64::ln(1.0 - output);
        self.loss += loss;

        self.amount += 1.0;

        loss
    }

    pub fn get_log_loss_avg(&self) -> f64 {
        -self.loss / self.amount
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, radius: f32, output: Option<f64>) {
        let font_size = radius as i32 / 3;

        let color = match output {
            Some(output) => {
                let r = ((1.0 - output) * 255.0) as u8;
                let g = (output * 255.0) as u8;
                let b = ((1.0 - output) * 255.0) as u8;
                raylib::color::Color::new(r, g, b, 255)
            }
            None => raylib::color::Color::new(255, 255, 255, 255),
        };

        let line_color = raylib::color::Color::new(0, 0, 0, 255);

        let size = 60 * (self.inputs - 1) as i32;

        let start = y - size / 2;

        for i in 0..self.inputs {
            let x_line = x - 3 * radius as i32;
            let y_line = start + radius as i32 * i as i32;

            d.draw_line(x_line, y_line, x, y, line_color);

            let data = self.data.get(i).unwrap_or(&0.0);

            let data_text = format!("{:.2} * {:.2}", data, self.weights[i]);
            let text_width = d.measure_text(&data_text, font_size);

            d.draw_text(
                &data_text,
                x_line - text_width - 3,
                y_line - font_size / 2,
                font_size,
                Color::BLACK,
            );
        }

        let bias = format!("{:.2}", self.biase);

        let bias_width = d.measure_text(&bias, font_size);

        d.draw_text(
            &bias,
            x - (bias_width / 2 as i32),
            y + radius as i32 + 5,
            font_size,
            Color::BLACK,
        );

        d.draw_circle(x, y, radius, color);

        d.draw_circle_lines(x, y, radius, Color::BLACK);

        if let Some(output) = output {
            let output_text = format!("{:.2}", output);
            let output_width = d.measure_text(&output_text, font_size);

            d.draw_text(
                &output_text,
                x - output_width / 2,
                y - font_size / 2,
                font_size,
                Color::BLACK,
            );
        }
    }
}
