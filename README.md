# Spatial Vision — Full-stack Rust/WASM

Prototipe deteksi gestur pinch tangan via kamera: backend Axum melayani asset
statis, frontend WebAssembly (Rust) menghitung jarak ibu jari–telunjuk dari
landmark MediaPipe, lalu menampilkan status pinch di layar.

## Prasyarat
- Rust stable + `wasm32-unknown-unknown`
- `wasm-pack` (`cargo install wasm-pack`)
- Node tidak diperlukan

## Menjalankan
```bash
rustup target add wasm32-unknown-unknown
cd frontend
wasm-pack build --target web --out-dir dist/pkg
cp index.html style.css dist/
cd ../backend
cargo run
```
Buka `http://localhost:3000` (kamera hanya berjalan di HTTPS atau localhost).

## Arsitektur
| Komponen | Path | Peran |
|----------|------|-------|
| Backend | `backend/src/main.rs` | Axum — serve `frontend/dist/` di port 3000 |
| Frontend | `frontend/src/lib.rs` | `detect_pinch` + `parse_pinch` + 5 unit test |
| MediaPipe | `index.html` | Pipeline landmark dari kamera via CDN |

Format input `detect_pinch(landmarks_json, was_pinching)` — array tangan dari
`multiHandLandmarks` — `[[[x,y,z]x21], [[x,y,z]x21]]`, plus state pinch frame
sebelumnya (untuk hysteresis di Rust). Output JSON:
`{"isPinching":bool,"strength":float,"gesture":"pinch"|"palm"|"none"}`.

- **Hysteresis ambang ganda:** pinch ON `< 0.045`, OFF `> 0.06`, di antara =
  tahan state — anti-jitter saat tangan goyang di ambang.
- **Gesture `palm`:** jarak wrist(0)–middle tip(12) `> 0.40` (normalized) dan
  tidak sedang pinch.
- **FPS counter:** tampil di badge status, rolling 1 detik.

## Testing
```bash
cargo test --manifest-path frontend/Cargo.toml
```
5 unit test: pinch dekat, tangan terbuka (palm), hysteresis di deadband, JSON
invalid, tanpa tangan.

## Catatan
- Gesture dihitung murni di sisi klien; kamera tidak pernah dikirim ke backend.
- Untuk produksi: pindah MediaPipe ke WASM task API (tanpa CDN) + HTTPS.
