use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Landmark {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[wasm_bindgen]
pub fn from_landmarks_rust_array(array: &[f32]) -> Result<Vec<Landmark>, JsValue> {
    if array.len() != 63 {
        return Err(JsValue::from_str("Array must have 63 elements (3 * 21 landmarks)"));
    }

    let mut landmarks = Vec::new();
    for i in 0..21 {
        let idx = i * 3;
        landmarks.push(Landmark {
            x: array[idx],
            y: array[idx + 1],
            z: array[idx + 2],
        });
    }
    Ok(landmarks)
}

#[wasm_bindgen]
pub fn detect_pinch(landmarks: &[Landmark]) -> (bool, f32) {
    let left_hand = &landmarks[0..21];
    let right_hand = &landmarks[21..42];

    let left_index = left_hand[8];
    let left_thumb = left_hand[4];
    let right_index = right_hand[8];
    let right_thumb = right_hand[4];

    let left_sq = (left_index.x - left_thumb.x).powi(2) +
                  (left_index.y - left_thumb.y).powi(2) +
                  (left_index.z - left_thumb.z).powi(2);
    let right_sq = (right_index.x - right_thumb.x).powi(2) +
                   (right_index.y - right_thumb.y).powi(2) +
                   (right_index.z - right_thumb.z).powi(2);

    let min_dist = left_sq.min(right_sq).sqrt();
    let is_pinching = min_dist < 0.05;
    let strength = 1.0 - (min_dist / 0.15).clamp(0.0, 1.0);

    (is_pinching, strength)
}
