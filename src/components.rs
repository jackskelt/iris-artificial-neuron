use raylib::prelude::*;

use crate::State;

pub fn iris_type_box(d: &mut RaylibDrawHandle, state: &mut State, rect: Rectangle) -> Option<i32> {
    let collision = rect.check_collision_point_rec(d.get_mouse_position());
    let pressed = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    let last_result = state.iris_select.1;

    if collision && pressed {
        state.iris_select.0 = !state.iris_select.0;
    }

    d.gui_dropdown_box(
        Rectangle::new(120.0, 0.0, 80.0, 30.0),
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
    let gap = 10;
    let mut y = rect.y;
    let biggest_size = d.measure_text("Iris Versicolour", font_size);

    let columns = [
        "Specie",
        "Sepal Length",
        "Sepal Width",
        "Petal Length",
        "Petal Width",
    ];

    for flower in state.test_data.iter() {
        let text = flower.1.to_string();

        d.draw_text(&text, rect.x as i32, y as i32, font_size, Color::BLACK);

        // Draw the values from flower.0

        let range = match state.iris_select.1 {
            0 => 0..2,
            1 => 2..4,
            _ => 0..4,
        };

        let mut size = 0;

        let text_size = d.measure_text("9.99", font_size);

        for i in range {
            // Draw the column with the column name
            let text = format!("{:.2}", flower.0[i]);

            d.draw_text(
                &text,
                rect.x as i32 + biggest_size + size as i32,
                y as i32,
                font_size,
                Color::BLACK,
            );

            size += text_size;

            size += gap;
        }

        y += font_size as f32;
    }
}
