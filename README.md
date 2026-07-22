# Spatial Vision — Full-stack Rust/WASM

Prototipe ini menyediakan backend Axum, frontend WebAssembly Rust, akses kamera, canvas spatial UI, serta modul Rust murni untuk perhitungan pinch.

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

## Catatan implementasi
- `frontend/src/gesture.rs` berisi `distance`, `is_pinching`, dan `pinch_strength`; indeks landmark mengikuti MediaPipe (4 ibu jari, 8 telunjuk).
- `frontend/src/render.rs` menggambar panel interaktif pada canvas.
- `backend/src/main.rs` melayani asset statis dan menyediakan `/ws` broadcast JSON antarklien.
- Untuk produksi, tambahkan pipeline MediaPipe JS (atau WASM vision task) di `index.html` dan kirim 21 landmark ke fungsi gesture Rust melalui `wasm-bindgen`; backend tidak pernah menerima frame kamera mentah.

## Pemeriksaan
Struktur source sengaja minim dan tanpa dependensi framework UI agar mudah dibangun. Jalankan `cargo fmt --all` dan `cargo check` setelah Rust tersedia.
