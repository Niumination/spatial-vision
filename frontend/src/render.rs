use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub fn draw_scene(canvas: &HtmlCanvasElement, x: f32, y: f32, pinching: bool) -> Result<(), JsValue> {
    let ctx = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;
    let w = canvas.width() as f64; let h = canvas.height() as f64;
    ctx.set_fill_style_str("#07111f");
    ctx.fill_rect(0.0, 0.0, w, h);
    let size = 100.0;
    let px = (x as f64 * w).clamp(size / 2.0, w - size / 2.0);
    let py = (y as f64 * h).clamp(size / 2.0, h - size / 2.0);
    ctx.save();
    ctx.translate(px, py)?;
    ctx.set_fill_style_str(if pinching { "#ffb84d" } else { "#4d9cff" });
    ctx.fill_rect(-size / 2.0, -size / 2.0, size, size);
    ctx.set_stroke_style_str("#e8f3ff");
    ctx.set_line_width(3.0);
    ctx.stroke_rect(-size / 2.0, -size / 2.0, size, size);
    ctx.restore();
    Ok(())
}
