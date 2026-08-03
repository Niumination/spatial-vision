use wasm_bindgen::prelude::*;
use serde_json::Value;

// Threshold hysteresis (ambang ganda) — anti-jitter saat tangan goyang di ambang.
// ON  < 0.045, OFF > 0.06, di antara = pertahankan state sebelumnya.
const PINCH_ON: f64 = 0.045;
const PINCH_OFF: f64 = 0.06;
const PINCH_RANGE: f64 = 0.15; // strength memudar linear hingga jarak ini

// Open palm: jarak wrist(0)–middle tip(12) di atas ambang ini (normalized).
const PALM_MIN_ARM: f64 = 0.40;

/// Wrapper wasm-bindgen — lempar ke JS, ditangkap try/catch di index.html.
/// `was_pinching`: state pinch frame sebelumnya (dikelola JS) untuk hysteresis.
#[wasm_bindgen]
pub fn detect_pinch(landmarks_json: &str, was_pinching: bool) -> Result<String, String> {
    parse_pinch(landmarks_json, was_pinching)
}

/// Deteksi gestur dari landmark MediaPipe. Input: array tangan, tiap tangan
/// 21 landmark `[x,y,z]` (format sama dengan output multiHandLandmarks).
/// Output JSON: `{"isPinching":bool,"strength":float,"gesture":"pinch"|"palm"|"none"}`.
pub fn parse_pinch(landmarks_json: &str, was_pinching: bool) -> Result<String, String> {
    match serde_json::from_str::<Value>(landmarks_json) {
        Ok(Value::Array(hands)) => {
            if hands.is_empty() {
                return Err("No hands".into());
            }
            let mut min_dist = f64::MAX;
            let mut palm_arm = 0.0_f64;
            for hand in hands.iter() {
                if let (Some(thumb), Some(index)) = (point(hand, 4), point(hand, 8)) {
                    min_dist = min_dist.min(dist3d(thumb, index));
                }
                if let (Some(wrist), Some(mid_tip)) = (point(hand, 0), point(hand, 12)) {
                    palm_arm = palm_arm.max(dist3d(wrist, mid_tip));
                }
            }
            // Hysteresis: naik-turun pakai ambang berbeda, tengah = tahan state.
            let is_pinching = if min_dist < PINCH_ON {
                true
            } else if min_dist > PINCH_OFF {
                false
            } else {
                was_pinching
            };
            let strength = (1.0 - (min_dist / PINCH_RANGE).min(1.0)).max(0.0);
            let is_palm = palm_arm > PALM_MIN_ARM;
            let gesture = if is_pinching {
                "pinch"
            } else if is_palm {
                "palm"
            } else {
                "none"
            };
            Ok(format!(
                "{{\"isPinching\":{},\"strength\":{:.3},\"gesture\":\"{}\"}}",
                is_pinching, strength, gesture
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
        l[0][8] = [0.31, 0.31, 0.0]; // telunjuk — jarak ~0.014 < 0.045
        serde_json::to_value(l).unwrap()
    }

    /// Tangan terbuka lebar: wrist & middle tip berjauhan, bukan pinch.
    fn hand_open() -> serde_json::Value {
        let mut l: Vec<Vec<[f64; 3]>> = vec![vec![[0.5, 0.5, 0.0]; 21]];
        l[0][4] = [0.1, 0.1, 0.0];
        l[0][8] = [0.9, 0.9, 0.0]; // jarak ~1.13 >> 0.06
        l[0][0] = [0.5, 0.8, 0.0]; // wrist bawah
        l[0][12] = [0.5, 0.1, 0.0]; // middle tip atas — arm ~0.7 > 0.40
        serde_json::to_value(l).unwrap()
    }

    /// Tangan dalam zona abu-abu (0.045–0.06) — untuk uji hysteresis.
    fn hand_in_deadband() -> serde_json::Value {
        let mut l: Vec<Vec<[f64; 3]>> = vec![vec![[0.5, 0.5, 0.0]; 21]];
        l[0][4] = [0.30, 0.30, 0.0];
        l[0][8] = [0.335, 0.335, 0.0]; // jarak ~0.0495 — di dalam deadband
        serde_json::to_value(l).unwrap()
    }

    #[test]
    fn pinch_detected_when_close() {
        let out = parse_pinch(&hand_pinching().to_string(), false).unwrap();
        assert!(out.contains("\"isPinching\":true"), "got {out}");
        assert!(out.contains("\"gesture\":\"pinch\""), "got {out}");
        assert!(out.contains("\"strength\":0.9"), "got {out}");
    }

    #[test]
    fn no_pinch_when_far() {
        let out = parse_pinch(&hand_open().to_string(), true).unwrap();
        assert!(out.contains("\"isPinching\":false"), "got {out}");
        assert!(out.contains("\"gesture\":\"palm\""), "got {out}");
    }

    #[test]
    fn hysteresis_holds_in_deadband() {
        let on = parse_pinch(&hand_in_deadband().to_string(), true).unwrap();
        assert!(on.contains("\"isPinching\":true"), "hold ON: got {on}");
        let off = parse_pinch(&hand_in_deadband().to_string(), false).unwrap();
        assert!(off.contains("\"isPinching\":false"), "hold OFF: got {off}");
    }

    #[test]
    fn rejects_invalid_json() {
        assert!(parse_pinch("not json", false).is_err());
    }

    #[test]
    fn rejects_empty_hands() {
        assert!(parse_pinch("[]", false).is_err());
    }
}
