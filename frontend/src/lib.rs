use wasm_bindgen::prelude::*;
use serde_json::Value;

#[wasm_bindgen]
pub fn detect_pinch(landmarks_json: &str) -> Result<String, JsValue> {
    match serde_json::from_str::<Value>(landmarks_json) {
        Ok(Value::Array(landmarks_raw)) => {
            if landmarks_raw.len() < 2 {
                return Err(JsValue::from_str("Need 2 hands"));
            }

            let left_index = landmarks_raw.get(0).and_then(|h| h.as_array()).and_then(|a| a.get(8)).and_then(|v| v.as_f64()).unwrap_or(0.5);
            let left_thumb = landmarks_raw.get(0).and_then(|h| h.as_array()).and_then(|a| a.get(4)).and_then(|v| v.as_f64()).unwrap_or(0.5);
            let right_index = landmarks_raw.get(1).and_then(|h| h.as_array()).and_then(|a| a.get(8)).and_then(|v| v.as_f64()).unwrap_or(0.5);
            let right_thumb = landmarks_raw.get(1).and_then(|h| h.as_array()).and_then(|a| a.get(4)).and_then(|v| v.as_f64()).unwrap_or(0.5);

            let left_sq = (left_index - left_thumb).powi(2);
            let right_sq = (right_index - right_thumb).powi(2);
            let min_dist = (left_sq.min(right_sq)).sqrt();
            let is_pinching = min_dist < 0.05;
            let strength = (1.0 - (min_dist / 0.15).min(1.0)) as f64;

            Ok(format!("{{\"isPinching\":{},\"strength\":{}}}", is_pinching, strength))
        }
        _ => Err(JsValue::from_str("Invalid JSON format")),
    }
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}
