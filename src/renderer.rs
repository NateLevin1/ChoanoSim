use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::get_simulator;

fn clear_canvas(context: &CanvasRenderingContext2d) {
    let canvas = context.canvas().unwrap();
    context.set_fill_style(&"rgb(26, 101, 171)".into());
    context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn render_simulator(context: CanvasRenderingContext2d, camera_x: i32, camera_y: i32) {
    clear_canvas(&context);

    let simulator = get_simulator();

    // draw food
    context.set_fill_style(&"rgba(58, 29, 0, 0.5)".into());
    for food_row in simulator.get_food() {
        for food in food_row {
            if let Some(food) = food {
                let x = (food.x as i32 + camera_x) as f64;
                let y = (food.y as i32 + camera_y) as f64;
                let size = 2.5;
                context.save();
                context.begin_path();
                context.arc(x, y, size, 0.0, 2.0 * 3.14).unwrap();
                context.fill();
                context.close_path();
                context.restore();
            }
        }
    }

    // draw cells
    context.set_line_width(3.0);
    for cell in simulator.get_cells() {
        // make sure cell is in bounds, if not skip rendering
        let cell_camera_dist = cell.genes.size as i32 + 10;
        if (cell.x as i32 + cell_camera_dist) < -camera_x
            || (cell.y as i32 + cell_camera_dist) < -camera_y
        {
            continue;
        }

        context.set_stroke_style(&"rgb(100, 220, 255)".into());
        if cell.remaining_steps_until_child_born != 0 {
            // if holding a child, change fill color
            let amount_close_to_birth = (cell.remaining_steps_until_child_born as f64)
                / cell.genes.steps_until_child_born
                * 200.0;
            context.set_fill_style(
                &format!(
                    "rgb({}, {}, {})",
                    0,
                    amount_close_to_birth,
                    200.0 - amount_close_to_birth,
                )
                .into(),
            );
        } else {
            context.set_fill_style(&(&cell.color).into());
        }
        let x = (cell.x as i32 + camera_x) as f64;
        let y = (cell.y as i32 + camera_y) as f64;
        let size = (cell.genes.size) as f64;
        context.save();
        context.begin_path();
        context.translate(x, y).unwrap();
        context.rotate(cell.radians - 3.14).unwrap();
        context.arc(0.0, 0.0, size, 0.2 * 3.14, 1.8 * 3.14).unwrap();
        context.translate(0.0, -size * 1.0 / 2.0).unwrap();
        context.line_to(size + 7.0, 0.0);
        context.translate(0.0, size).unwrap();
        context.line_to(size + 7.0, 0.0);
        context.translate(0.0, 0.0).unwrap();
        context.line_to(size - 7.0, 4.0);
        context.fill();
        context.stroke();
        context.close_path();
        context.restore();

        context.set_stroke_style(&"rgb(0, 0, 0)".into());
        context.set_line_width(1.0);
        context.set_fill_style(
            &format!("rgb({}, {}, {})", (cell.get_fullness() * 255.0), 0.0, 0.0).into(),
        );
        context.begin_path();
        context
            .arc(x, y, cell.genes.stomach_size, 0.0, 2.0 * 3.14)
            .unwrap();
        context.fill();
        context.stroke();
        context.close_path();

        context.set_line_width(3.0);
        context.set_stroke_style(&"rgb(50, 200, 255)".into());
        context.begin_path();
        let fla_radius = (size / 2.0) * cell.genes.flagellum_size;
        let angle = 3.15f64
            * ((cell.display_seed * 360.0 + simulator.get_steps() as f64) * 30.0 % 360.0)
            / 180.0;
        context.move_to(x, y);
        context.line_to(x + fla_radius * angle.cos(), y + fla_radius * angle.sin());
        context.stroke();
        context.close_path();
        context.restore();
    }

    // draw outline
    context.set_stroke_style(&"rgb(255, 255, 255)".into());
    context.set_line_width(2.0);
    context.stroke_rect(
        camera_x as f64,
        camera_y as f64,
        simulator.get_config().width as f64,
        simulator.get_config().height as f64,
    );
}
