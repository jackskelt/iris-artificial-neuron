use crate::{
    components::{data_values, draw_table, frequency, iris_data_type_box, iris_type_box},
    state::{update_data, update_data_type, update_iris_type},
    State, WINDOW_WIDTH,
};
use raylib::prelude::*;

pub fn update(d: &mut RaylibDrawHandle, state: &mut State) {
    d.clear_background(raylib::prelude::Color::WHITE);

    d.draw_fps(0, 0);

    // Separation graph
    state.separation_graph.draw(d);

    // Line graph
    state.line_graph.draw(d);

    d.draw_text(
        &format!("Log loss: {:.3}", state.loss),
        700,
        360,
        20,
        Color::BLACK,
    );

    data_values(d, state);

    draw_table(d, state, Rectangle::new(400.0, 420.0, 300.0, 300.0));

    // Inputs
    if d.is_key_pressed(KeyboardKey::KEY_P) {
        state.pause = !state.pause;
    }

    let data_type_selected = iris_data_type_box(d, state, Rectangle::new(120.0, 0.0, 80.0, 30.0));
    frequency(d, state, Rectangle::new(480.0, 680.0, 80.0, 30.0));

    let iris_type_selected = iris_type_box(d, state, Rectangle::new(210.0, 0.0, 100.0, 30.0));

    // Neuron
    state.neuron.draw(d, 700, 190, 60.0, Some(state.outputs.0));

    // Type change
    if let Some(iris_type) = iris_type_selected {
        update_iris_type(state, iris_type);
        update_data_type(state, state.iris_select.1);
    }
    if let Some(selected) = data_type_selected {
        update_data_type(state, selected);
    }

    // Update data
    if state.pause {
        d.draw_text("Paused", WINDOW_WIDTH as i32 - 80, 0, 20, Color::RED);
        return;
    }

    if d.get_time() > state.last_event_time + (1.0 / state.update_hz as f64) {
        update_data(state);
        state.last_event_time = d.get_time();
    }
}
