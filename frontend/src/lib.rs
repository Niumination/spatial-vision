use wasm_bindgen::prelude::*;
use serde_json::Value;

/// Wrapper wasm-bindgen — lempar ke JS, ditangkap try/catch di index.html.
#[wasm_bindgen]
pub fn detect_pinch(landmarks_json: &str) -> Result<String, String> {
    parse_pinch(landmarks_json)
}

/// Deteksi pinch dari landmark MediaPipe. Input: array tangan, tiap tangan
/// 21 landmark `[x,y,z]` (format sama dengan output multiHandLandmarks).
pub fn parse_pinch(landmarks_json: &str) -> Result<String, String> {
    match serde_json::from_str::<Value>(landmarks_json) {
        Ok(Value::Array(hands)) => {
            if hands.is_empty() {
                return Err("No hands".into());
            }
            let mut min_dist = f64::MAX;
            for hand in hands.iter() {
                if let (Some(thumb), Some(index)) = (point(hand, 4), point(hand, 8)) {
                    min_dist = min_dist.min(dist3d(thumb, index));
                }
            }
            let is_pinching = min_dist < 0.05;
            let strength = (1.0 - (min_dist / 0.15).min(1.0)).max(0.0);
            Ok(format!(
                "{{\"isPinching\":{},\"strength\":{:.3}}}",
                is_pinching, strength
            ))
        }
        _ => Err("Invalid JSON format".into()),
    }
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Ambil landmark index `idx` sebagai titik [x, y, z].
fn point(hand: &Value, idx: usize) -> Option<[f64; 3]> {
    let l = hand.as_array()?.get(idx)?.as_array()?;
    Some([
        l.get(0)?.as_f64()?,
        l.get(1)?.as_f64()?,
        l.get(2)?.as_f64()?,
    ])
}

fn dist3d(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tangan dengan jempol & telunjuk berdekatan → pinch.
    fn hand_pinching() -> serde_json::Value {
        let mut l: Vec<Vec<[f64; 3]>> = vec![vec![[0.5, 0.5, 0.0]; 21]];
        l[0][4] = [0.30, 0.30, 0.0]; // ibu jari
        l[0][8] = [0.31, 0.31, 0.0]; // telunjuk — jarak ~0.014 < 0.05
        serde_json::to_value(l).unwrap()
    }

    /// Tangan dengan jempol & telunjuk berjauhan → bukan pinch.
    fn hand_open() -> serde_json::Value {
        let mut l: Vec<Vec<[f64; 3]>> = vec![vec![[0.5, 0.5, 0.0]; 21]];
        l[0][4] = [0.1, 0.1, 0.0];
        l[0][8] = [0.9, 0.9, 0.0]; // jarak ~1.13 >> 0.15
        serde_json::to_value(l).unwrap()
    }

    #[test]
    fn pinch_detected_when_close() {
        let out = parse_pinch(&hand_pinching().to_string()).unwrap();
        assert!(out.contains("\"isPinching\":true"), "got {out}");
        assert!(out.contains("\"strength\":0.9"), "got {out}");
    }

    #[test]
    fn no_pinch_when_far() {
        let out = parse_pinch(&hand_open().to_string()).unwrap();
        assert!(out.contains("\"isPinching\":false"), "got {out}");
    }

    #[test]
    fn rejects_invalid_json() {
        assert!(parse_pinch("not json").is_err());
    }

    #[test]
    fn rejects_empty_hands() {
        assert!(parse_pinch("[]").is_err());
    }
}
