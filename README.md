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
| Frontend | `frontend/src/lib.rs` | `detect_pinch` + `init` (ekspor wasm) |
| Logika murni | `frontend/src/lib.rs` | `parse_pinch` — jarak 3D ibu jari (4) & telunjuk (8) |
| MediaPipe | `index.html` | Pipeline landmark dari kamera via CDN |

Format input `detect_pinch`: array tangan dari `multiHandLandmarks` —
`[[[x,y,z]x21], [[x,y,z]x21]]`. Output JSON: `{"isPinching":bool,"strength":float}`.

## Testing
```bash
cargo test --manifest-path frontend/Cargo.toml
```
4 unit test: pinch dekat, tangan terbuka, JSON invalid, tanpa tangan.

## Catatan
- Gesture dihitung murni di sisi klien; kamera tidak pernah dikirim ke backend.
- `process_pinch_data` di `mediapipe.rs` belum terpakai oleh `index.html`
  (candidate hapus jika tidak dipakai lanjut).
- Untuk produksi: pindah MediaPipe ke WASM task API (tanpa CDN) + HTTPS.
