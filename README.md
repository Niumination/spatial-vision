# Spatial Vision — Full-stack Rust/WASM

Spatial launcher berbasis kamera: kursor tangan (ujung telunjuk), pinch = klik,
palm = tutup modal, swipe kiri/kanan = pindah halaman, dwell 900ms = pilih kartu.
Backend Axum melayani asset statis, frontend WebAssembly (Rust) menghitung jarak
ibu jari–telunjuk dari landmark MediaPipe (self-host, offline), lalu menampilkan
status pinch di layar. Video kamera **tidak pernah** dikirim ke backend.

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
| MediaPipe | `frontend/vendor/mediapipe-tasks/` | Pipeline landmark — self-host offline (11MB, di-track) |

Format input `detect_pinch(landmarks_json, was_pinching)` — array tangan dari
`multiHandLandmarks` — `[[[x,y,z]x21], [[x,y,z]x21]]`, plus state pinch frame
sebelumnya (untuk hysteresis di Rust). Output JSON:
`{"isPinching":bool,"strength":float,"gesture":"pinch"|"palm"|"none"}`.

- **Hysteresis ambang ganda:** pinch ON `< 0.045`, OFF `> 0.06`, di antara =
  tahan state — anti-jitter saat tangan goyang di ambang.
- **Gesture `palm`:** jarak wrist(0)–middle tip(12) `> 0.40` (normalized) dan
  tidak sedang pinch.
- **FPS counter:** tampil di badge status, rolling 1 detik.

## Launcher & Interaksi

| Fitur | Cara kerja |
|-------|-----------|
| **Kursor + klik** | Kursor dari ujung telunjuk [8] (x di-mirror), smoothing 0.6; rising-edge pinch = klik pada kartu |
| **Swipe navigasi** | Window 220ms pada ujung telunjuk [8], `|dx| > 0.22`, rasio kelurusan `> 0.45`, cooldown 500ms → pindah halaman + dot indikator |
| **App modal** | Klik kartu → modal dengan judul, deskripsi, tombol "Buka <app>" (link URL) + "Tutup" |
| **Dwell-to-select** | Tahan kursor di kartu 900ms = pilih (progress bar hijau via CSS) — alternatif klik |
| **Palm = tutup** | Gestur telapak terbuka menutup modal yang terbuka |
| **Dual input** | Mouse juga bisa klik kartu langsung (memudahkan testing tanpa kamera) |
| **Keyboard** | `←` / `→` pindah halaman, `Enter` klik kartu hover, `Esc` tutup modal |
| **Sentuh (mobile)** | Swipe kiri/kanan di area `.stage` (threshold 40px) = pindah halaman |
| **Loading screen** | Overlay + spinner saat WASM + MediaPipe inisialisasi, hilang otomatis (juga saat MediaPipe gagal dimuat) |

## Testing
```bash
cargo test --manifest-path frontend/Cargo.toml
```
5 unit test: pinch dekat, tangan terbuka (palm), hysteresis di deadband, JSON
invalid, tanpa tangan.

CI: GitHub Actions (`.github/workflows/ci.yml`) — build Rust backend, build WASM
frontend, `cargo test`.

## Catatan
- Gesture dihitung murni di sisi klien; kamera tidak pernah dikirim ke backend.
- MediaPipe di-self-host di `frontend/vendor/mediapipe-tasks/` (11MB) — berjalan offline, tanpa CDN.
- Untuk produksi: pindah MediaPipe ke WASM task API (lebih kecil) + HTTPS + deploy.
