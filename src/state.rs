use crate::{
    neuron,
    separation_graph::SeparationGraph,
    train_data::{Iris, IrisData},
    State,
};
use rand::seq::SliceRandom;
use raylib::prelude::*;

pub fn update_data_type(state: &mut State, selected: i32) {
    match selected {
        0 => {
            // Sepal
            let graph_data = IrisData::get_graph_data(&state.data);

            let sepal_data = graph_data
                .iter()
                .map(|(x, y, _, _, c)| (*x, *y, *c))
                .collect::<Vec<(f32, f32, Color)>>();

            state.separation_graph = SeparationGraph::new(
                40.0,
                380.0,
                300.0,
                300.0,
                "SepalLengthCm".to_owned(),
                "SepalWidthCm".to_owned(),
                sepal_data,
            );
            state.neuron = neuron::Neuron::new(2, neuron::SIGMOID, 0.5);

            state.train_data = state
                .data
                .iter()
                .map(|d| (vec![d.sepal_length, d.sepal_width], d.species.into()))
                .collect();
        }
        1 => {
            let graph_data = IrisData::get_graph_data(&state.data);

            let petal = graph_data
                .iter()
                .map(|(_, _, x, y, c)| (*x, *y, *c))
                .collect::<Vec<(f32, f32, Color)>>();

            state.separation_graph = SeparationGraph::new(
                40.0,
                380.0,
                300.0,
                300.0,
                "PetalLengthCm".to_owned(),
                "PetalWidthCm".to_owned(),
                petal,
            );

            state.neuron = neuron::Neuron::new(2, neuron::SIGMOID, 0.5);

            state.train_data = state
                .data
                .iter()
                .map(|d| (vec![d.petal_length, d.petal_width], d.species.into()))
                .collect();
        } // Petal
        2 => {
            state.separation_graph = SeparationGraph::new(
                40.0,
                380.0,
                300.0,
                300.0,
                "".to_owned(),
                "".to_owned(),
                vec![],
            );

            state.neuron = neuron::Neuron::new(4, neuron::SIGMOID, 0.5);

            state.train_data = state
                .data
                .iter()
                .map(|d| {
                    (
                        vec![d.sepal_length, d.sepal_width, d.petal_length, d.petal_width],
                        d.species.into(),
                    )
                })
                .collect();
        } // All
        _ => {}
    }

    state.line_graph.clear_data();
    state.pause = true;
    state.generations = 0;
    state.data_index = 0;
}

pub fn update_iris_type(state: &mut State, iris_type: Iris) {
    match iris_type {
        Iris::Versicolour => {
            state.data = IrisData::get_data(&state.csv_data, Iris::Versicolour);
        }
        Iris::Virginica => {
            state.data = IrisData::get_data(&state.csv_data, Iris::Virginica);
        }
        _ => {}
    }
    state.data.shuffle(&mut rand::thread_rng());
    state.test_data = IrisData::get_test_data(&state.data, 3);
}

pub fn update_data(state: &mut State) {
    let output = state
        .neuron
        .feed_forward(&state.train_data[state.data_index].0);
    let (error, gradient) = state
        .neuron
        .back_propagate(output, state.train_data[state.data_index].1);

    state.outputs = (output, error, gradient);

    state.target = Some(state.train_data[state.data_index].1);

    if state.neuron.inputs == 2 {
        state
            .separation_graph
            .set_decision_line(&state.neuron.weights, state.neuron.biase)
    }

    state.loss = state
        .neuron
        .log_loss(output, state.train_data[state.data_index].1)
        .abs();

    state.line_graph.add_data(state.neuron.get_log_loss_avg());

    match state.iris_select.1 {
        0 => {}
        1 => {}
        2 => {}
        _ => {}
    }

    state.data_index = (state.data_index + 1) % state.train_data.len();

    if state.data_index == 0 {
        state.generations += 1;
    }
}
