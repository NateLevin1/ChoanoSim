use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

use crate::{get_simulator};

#[allow(dead_code)]
#[wasm_bindgen]
pub fn initialize_canvas(context: CanvasRenderingContext2d) {
    clear_canvas(&context);
}

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
    
    // draw cells
    context.set_stroke_style(&"rgb(100, 255, 255)".into());
    context.set_line_width(1.5);
    for cell in simulator.get_cells() {
        context.set_fill_style(&JsValue::from_str(&cell.color));
        let x = (cell.x as i32 + camera_x) as f64;
        let y = (cell.y as i32 + camera_y) as f64;
        let size = (cell.size) as f64;
        context.save();
        context.begin_path();
        context.translate(x, y).unwrap();
        context.rotate(cell.radians - 3.14).unwrap();
        context.arc(0.0, 0.0, size, 0.2 * 3.14, 1.8 * 3.14).unwrap();
        context.translate(0.0, -size * 1.0 / 2.0).unwrap();
        context.line_to(size+7.0, 0.0);
        context.translate(0.0, size).unwrap();
        context.line_to(size+7.0, 0.0);
        context.translate(0.0, 0.0).unwrap();
        context.line_to(size-7.0, 4.0);
        context.fill();
        context.stroke();
        context.close_path();
        context.restore();

        context.set_stroke_style(&"rgb(50, 200, 255)".into());
        context.set_line_width(3.0);
        context.begin_path();
        let fla_radius = size * cell.flagellum_size;
        let angle = 3.15f64 * ((cell.seed * 360.0 + simulator.get_steps() as f64) * 30.0 % 360.0) / 180.0;
        context.move_to(x, y);
        context.line_to(x + fla_radius * angle.cos(), y + fla_radius * angle.sin());
        context.stroke();
        context.close_path();
        context.restore();
    }

    // draw outline
    context.set_stroke_style(&"rgb(255, 255, 255)".into());
    context.set_line_width(2.0);
    context.stroke_rect(camera_x as f64, camera_y as f64, simulator.get_config().width as f64, simulator.get_config().height as f64);
}
