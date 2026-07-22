use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Landmark {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn distance(a: Landmark, b: Landmark) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}
