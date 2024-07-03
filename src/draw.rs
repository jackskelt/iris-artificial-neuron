use crate::{
    components::{data_values, draw_table, frequency, iris_type_box},
    neuron,
    separation_graph::SeparationGraph,
    train_data::IrisData,
    State, WINDOW_WIDTH,
};
use raylib::prelude::*;

pub fn update(d: &mut RaylibDrawHandle, state: &mut State) {
    d.clear_background(raylib::prelude::Color::WHITE);

    d.draw_fps(0, 0);

    // if input_res {
    //     println!("{}", String::from_utf8_lossy(&state.text))
    // }
    // if clicked {
    //     println!("Aiaiaia");
    // }
    // Separation graph
    state.separation_graph.draw(d);

    // Line graph
    state.line_graph.draw(d);

    d.draw_text(
        &format!("{}", state.last_event_time),
        (WINDOW_WIDTH) as i32 - 120,
        5,
        20,
        Color::GREEN,
    );

    d.draw_text(
        &format!("{}", d.get_time()),
        (WINDOW_WIDTH) as i32 - 120,
        25,
        20,
        Color::GREEN,
    );

    d.draw_text(
        &format!("Log loss: {:.3}", state.loss),
        700,
        360,
        20,
        Color::BLACK,
    );

    data_values(d, state);

    draw_table(d, state, Rectangle::new(400.0, 380.0, 300.0, 300.0));

    // Inputs
    if d.is_key_pressed(KeyboardKey::KEY_P) {
        state.pause = !state.pause;
    }

    let selected = iris_type_box(d, state, Rectangle::new(120.0, 0.0, 80.0, 30.0));
    frequency(d, state, Rectangle::new(220.0, 0.0, 80.0, 30.0));

    // Neuron
    state.neuron.draw(d, 700, 190, 60.0, Some(state.outputs.0));

    // Type change
    if let Some(selected) = selected {
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
                    .map(|d| (vec![d.sepal_length_cm, d.sepal_width_cm], d.species.into()))
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
                    .map(|d| (vec![d.petal_length_cm, d.petal_width_cm], d.species.into()))
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
                            vec![
                                d.sepal_length_cm,
                                d.sepal_width_cm,
                                d.petal_length_cm,
                                d.petal_width_cm,
                            ],
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

    // Update data
    if state.pause {
        d.draw_text("Paused", WINDOW_WIDTH as i32 - 80, 0, 20, Color::RED);
        return;
    }

    if d.get_time() > state.last_event_time + (1.0 / state.update_hz as f64) {
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

        state.last_event_time = d.get_time();
    }
}
