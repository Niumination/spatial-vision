use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let canvas: HtmlCanvasElement = document.get_element_by_id("scene").ok_or("#scene missing")?.dyn_into()?;
    let context = canvas.get_context("2d")?;
    let c = match context {
        Some(c) => c,
        None => return Err("canvas context not found".into()),
    };
    let ctx: CanvasRenderingContext2d = c.dyn_into()?;
    let w = canvas.width() as f64; let h = canvas.height() as f64;
    ctx.set_fill_style_str("#07111f");
    ctx.fill_rect(0.0, 0.0, w, h);
    ctx.set_fill_style_str("#4d9cff");
    ctx.fill_rect(100.0, 100.0, w - 200.0, h - 200.0);
    ctx.set_fill_style_str("#e8f3ff");
    ctx.set_font("30px system-ui");
    ctx.set_text_align("center");
    ctx.set_fill_style_str("#ffffff");
    let _ = ctx.fill_text("Spatial Vision - Minimal Test", w / 2.0, h / 2.0);
    Ok(())
}
