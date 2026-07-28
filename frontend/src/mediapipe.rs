use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_pinch_data(x: f32, y: f32, z: f32) -> f32 {
    let dist = (x * x + y * y + z * z).sqrt();
    if dist < 0.05 {
        1.0
    } else {
        0.0
    }
}
