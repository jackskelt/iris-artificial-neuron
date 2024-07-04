use raylib::prelude::*;

use crate::{train_data::Iris, State};

pub fn iris_data_type_box(
    d: &mut RaylibDrawHandle,
    state: &mut State,
    rect: Rectangle,
) -> Option<i32> {
    let collision = rect.check_collision_point_rec(d.get_mouse_position());
    let pressed = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    let last_result = state.iris_select.1;

    if collision && pressed {
        state.iris_select.0 = !state.iris_select.0;
    }

    d.gui_dropdown_box(
        rect,
        Some(rstr!("Sepal;Petal;All")),
        &mut state.iris_select.1,
        state.iris_select.0,
    );

    if last_result != state.iris_select.1 {
        state.iris_select.0 = false;
        Some(state.iris_select.1)
    } else {
        None
    }
}

pub fn iris_type_box(d: &mut RaylibDrawHandle, state: &mut State, rect: Rectangle) -> Option<Iris> {
    let collision = rect.check_collision_point_rec(d.get_mouse_position());
    let pressed = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    let last_result = state.iris_type_component.1;

    if collision && pressed {
        state.iris_type_component.0 = !state.iris_type_component.0;
    }

    d.gui_dropdown_box(
        rect,
        Some(rstr!("Virginica;Versicolor")),
        &mut state.iris_type_component.1,
        state.iris_type_component.0,
    );

    if last_result != state.iris_type_component.1 {
        state.iris_type_component.0 = false;
        match state.iris_type_component.1 {
            0 => state.iris_type = Iris::Virginica,
            1 => state.iris_type = Iris::Versicolour,
            _ => (),
        }
        Some(state.iris_type)
    } else {
        None
    }
}

pub fn frequency(d: &mut RaylibDrawHandle, state: &mut State, rect: Rectangle) {
    let collision = rect.check_collision_point_rec(d.get_mouse_position());
    let pressed = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    if collision && pressed {
        state.update_hz = state.update_hz;
    }

    d.gui_value_box(
        rect,
        Some(rstr!("Frequency (Hz)")),
        &mut state.update_hz,
        1,
        d.get_fps() as i32,
        state.pause && collision,
    );
}

pub fn data_values(d: &mut RaylibDrawHandle, state: &mut State) {
    d.draw_text(
        &format!("Output: {:.6}", state.outputs.0),
        700,
        300,
        20,
        Color::BLACK,
    );

    d.draw_text(
        &format!("Error: {:.6}", state.outputs.1),
        700,
        320,
        20,
        Color::BLACK,
    );

    d.draw_text(
        &format!("Gradient: {:.6}", state.outputs.2),
        700,
        340,
        20,
        Color::BLACK,
    );

    let generations_text = format!("Generations: {}", state.generations);
    let generations_size = d.measure_text(&generations_text, 20);

    d.draw_text(
        &format!("Generations: {}", state.generations),
        700 - generations_size / 2,
        40,
        20,
        Color::BLACK,
    );

    let data_size_text = format!("Data: {}/{}", state.data_index, state.train_data.len());
    let data_size_size = d.measure_text(&data_size_text, 20);

    d.draw_text(
        &format!("Data: {}/{}", state.data_index, state.train_data.len()),
        700 - data_size_size / 2,
        60,
        20,
        Color::BLACK,
    );

    if let Some(target) = state.target {
        let target_text = format!("Target: {}", target);
        let target_size = d.measure_text(&target_text, 20);

        d.draw_text(
            &format!("Target: {}", target),
            700 - target_size / 2,
            80,
            20,
            Color::BLACK,
        );
    }
}

pub fn draw_table(d: &mut RaylibDrawHandle, state: &mut State, rect: Rectangle) {
    // It will render a table for test_data
    let font_size = 20;
    let gap = 20;
    let mut y = rect.y;
    let biggest_size = d.measure_text("Iris Versicolor", font_size) + gap;
    let text_size = d.measure_text("9.99", font_size);

    let range = match state.iris_select.1 {
        0 => 0..2,
        1 => 2..4,
        _ => 0..4,
    };

    let mut columns = vec![
        ("Specie", biggest_size),
        ("SL", text_size),
        ("SW", text_size),
        ("PL", text_size),
        ("PW", text_size),
        ("O", text_size),
        ("%", text_size),
    ];

    let mut x = rect.x as i32;

    match state.iris_select.1 {
        0 => {
            columns = vec![
                ("Specie", biggest_size),
                ("SL", text_size),
                ("SW", text_size),
                ("O", text_size),
                ("%", text_size),
            ]
        }
        1 => {
            columns = vec![
                ("Specie", biggest_size),
                ("PL", text_size),
                ("PW", text_size),
                ("O", text_size),
                ("%", text_size),
            ]
        }
        _ => (),
    }

    for column in columns.iter() {
        d.draw_text(
            column.0,
            x,
            rect.y as i32 - font_size,
            font_size,
            Color::BLACK,
        );

        x += column.1 + gap;
    }

    for flower in state.test_data.iter() {
        let text = flower.1.to_string();

        d.draw_text(&text, rect.x as i32, y as i32, font_size, Color::BLACK);

        let mut size = 0;

        let mut inputs: Vec<f64> = vec![];

        for i in range.clone() {
            // Draw the column with the column name
            let text = format!("{:.2}", flower.0[i]);

            inputs.push(flower.0[i]);

            d.draw_text(
                &text,
                rect.x as i32 + biggest_size + size as i32,
                y as i32,
                font_size,
                Color::BLACK,
            );

            size += text_size + gap;
        }

        let output = state.neuron.feed_forward(&inputs);

        let output_text = format!("{:.2}", output);

        // Calc the percent of the output for the target
        let target: f64 = flower.1.into();
        let percent = 100.0 - (target - output).abs() * 100.0;

        let color = if percent < 50.0 {
            Color::RED
        } else {
            Color::GREEN
        };

        d.draw_text(
            &output_text,
            rect.x as i32 + biggest_size + size as i32,
            y as i32,
            font_size,
            color,
        );

        size += text_size;

        d.draw_text(
            &format!("{:.2}%", percent),
            rect.x as i32 + biggest_size + size as i32 + text_size,
            y as i32,
            font_size,
            color,
        );

        y += font_size as f32;
    }
}
