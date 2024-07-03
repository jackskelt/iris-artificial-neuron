use draw::update;
use rand::seq::SliceRandom;

mod components;
mod draw;
mod line_graph;
mod neuron;
mod separation_graph;
mod train_data;

use raylib::prelude::*;
use train_data::{Iris, IrisData};

pub const WINDOW_HEIGHT: f32 = 480.0 * 1.5;
pub const WINDOW_WIDTH: f32 = 640.0 * 1.5;

pub struct State {
    pub line_graph: line_graph::LineGraph,
    pub separation_graph: separation_graph::SeparationGraph,
    pub csv_data: Vec<IrisData>,
    pub data: Vec<IrisData>,
    pub test_data: Vec<(Vec<f64>, Iris)>,
    pub neuron: neuron::Neuron,
    pub text: [u8; 20],
    pub pause: bool,
    pub last_event_time: f64,
    pub update_hz: i32,
    pub iris_select: (bool, i32),
    pub train_data: Vec<(Vec<f64>, f64)>,
    pub data_index: usize,
    pub generations: usize,
    pub outputs: (f64, f64, f64),
    pub loss: f64,
    pub target: Option<f64>,
    pub iris_type: Iris,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        // .resizable()
        .title("Iris Artificial Neuron - Demo")
        .build();

    rl.set_target_fps(144);
    let csv_data = IrisData::read_csv();

    let mut data = IrisData::get_data(&csv_data, Iris::Virginica);

    let graph_data = IrisData::get_graph_data(&data);

    data.shuffle(&mut rand::thread_rng());

    // The test data will have 5 of each flower (setosa and virginica). Then it will be 5 of setosa and 5 of virginica

    let sepal_data = graph_data
        .iter()
        .map(|(x, y, _, _, c)| (*x, *y, *c))
        .collect::<Vec<(f32, f32, Color)>>();

    let train_data = data
        .iter()
        .map(|d| (vec![d.sepal_length_cm, d.sepal_width_cm], d.species.into()))
        .collect();

    let test_data = IrisData::get_test_data(&data, 3);

    let mut state: State = State {
        line_graph: line_graph::LineGraph::new(40.0, 40.0, 300.0, 300.0),
        separation_graph: separation_graph::SeparationGraph::new(
            40.0,
            380.0,
            300.0,
            300.0,
            "SepalLength".to_owned(),
            "SepalWidth".to_owned(),
            sepal_data,
        ),
        data,
        csv_data,
        test_data,
        neuron: neuron::Neuron::new(2, neuron::SIGMOID, 0.5),
        text: [0; 20],
        pause: true,
        last_event_time: 0.0,
        update_hz: 1,
        iris_select: (false, 0),
        train_data,
        data_index: 0,
        generations: 0,
        outputs: (0.0, 0.0, 0.0),
        target: None,
        loss: 0.0,
        iris_type: Iris::Virginica,
    };

    // state
    //     .separation_graph
    //     .set_decision_line(vec![100.0, 50.0], -30.0);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        update(&mut d, &mut state);
    }
}
