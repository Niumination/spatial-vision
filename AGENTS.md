# Spatial Vision — Project AGENTS.md

**Lokasi:** `sites/spatial-vision/`
**Stack:** Rust (Axum backend) + Rust/WASM (frontend), MediaPipe Hands via CDN
**Remote:** `git@github.com:Niumination/spatial-vision.git` (SSH only)
**Deploy:** ⚪ Prototype — lokal port 3000 (kamera butuh localhost/HTTPS)
**Status:** 🟢 Active — 3 Aug 2026

## Overview

Deteksi gestur pinch tangan via kamera. Backend Axum serve asset statis,
frontend WASM (Rust) menghitung jarak 3D ibu jari–telunjuk dari landmark
MediaPipe, lalu menampilkan status pinch di layar. **Seluruh proses gesture
murni di sisi klien — video kamera tidak pernah dikirim ke backend.**

## Struktur

```
spatial-vision/
├── backend/src/main.rs      Axum — ServeDir::new("../frontend/dist") di :3000
│                            (path relatif ke CWD → jalankan dari backend/)
├── frontend/
│   ├── src/lib.rs           detect_pinch (wasm wrapper) + parse_pinch (logika) + 5 unit test
│   ├── index.html           MediaPipe pipeline + kamera + error handling CDN
│   ├── style.css
│   └── dist/                Build artifact — ❌ TIDAK di-track (regenerable)
├── uploads/                 Draft spesifikasi
├── BUILD_STATUS.md          Catatan audit + ringkasan hasil
└── README.md                Panduan run + arsitektur
```

## Build & Test

```bash
# Prasyarat sekali
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# Build frontend
cd frontend
wasm-pack build --target web --out-dir dist/pkg
cp index.html style.css dist/

# Jalankan backend (WAJIB dari backend/, path serve relatif ke CWD)
cd ../backend
cargo run                      # → http://localhost:3000

# Unit test logika pinch
cargo test --manifest-path frontend/Cargo.toml
```

**Verifikasi:** `curl -I http://localhost:3000/` → 200, dan
`/pkg/spatial_vision_frontend_bg.wasm` → 200 (jangan hanya cek index.html —
glue WASM paling rawan 404).

## Aturan Proyek

1. **Logika gesture = murni di klien** — kamera tidak pernah ke backend; jangan
   tambah endpoint upload/video.
2. **YAGNI** — WebSocket/server push sudah pernah dihapus (tanpa klien jelas).
   Jangan tambah balik tanpa kebutuhan nyata.
3. **`parse_pinch` (logika murni) wajib di-test** — wrapper wasm `detect_pinch`
   harus tetap tipis (hanya lempar string, tangkap error di JS).
4. **Hysteresis ambang ganda** — pinch ON `< 0.045`, OFF `> 0.06`; state
   `was_pinching` dikelola JS, dihitung di Rust (stateless & testable).
5. **Repo hygiene — jangan commit artefak build**: `frontend/dist/`, `target/`,
   `*.wasm` di-ignore. Clone harus bisa rebuild via `wasm-pack` (glue tanpa wasm
   = app 404). Build dulu → commit hanya source.
6. **Backend path** — `ServeDir` relatif ke CWD; dokumentasikan menjalankan dari
   `backend/` agar tidak 404 (`../frontend/dist`).
7. Update README & BUILD_STATUS setiap ada perubahan arsitektur.

## Input/Output WASM

- **Input `detect_pinch(landmarks_json, was_pinching)`:** array tangan dari
  `multiHandLandmarks` — `[[[x,y,z]x21], [[x,y,z]x21]]` (normalized,
  multi-tangan; ambil jarak min) + state pinch frame sebelumnya.
- **Output JSON:** `{"isPinching":bool,"strength":float,"gesture":"pinch"|"palm"|"none"}`.
- Hysteresis: pinch ON `< 0.045`, OFF `> 0.06`, tengah = tahan state (anti-jitter).
- Gesture `palm`: jarak `wrist(0)–middle tip(12) > 0.40` saat tidak pinch.
- `strength` memudar linear hingga jarak 0.15.

## Catatan Perubahan

| Tanggal | Commit | Isi |
|---------|--------|-----|
| 3 Aug 2026 | *(belum di-commit)* | Gesture core polish: hysteresis, palm gesture, FPS counter, hapus mediapipe.rs |
| 3 Aug 2026 | `61c0b63` | Perf: lite model, 1 hand, cam 640x360, reset status |
| 3 Aug 2026 | `f763d79` | index.html rapi + error handling MediaPipe CDN + import `detect_pinch` langsung dari glue |
| 2 Aug 2026 | `43634a9` | pinch detection 3D + test, static path fix, **untrack dist** (repo hygiene) |
| 2 Aug 2026 | `71635b4` | Ponytail audit — hapus dead code, sederhanakan backend/frontend (-51 baris Rust, -9 deps) |
| 22 Jul 2026 | `03f329d` | gesture recognition + integrasi MediaPipe |
| 22 Jul 2026 | `19e8f76` | initial commit — full-stack Rust/WASM |

## Next Steps (kandidat)

- Kalibrasi threshold `PALM_MIN_ARM` & hysteresis via uji nyata di kamera
- Swipe kiri/kanan (butuh state antar-frame di JS) — navigasi
- Kursor tangan + pinch = klik (spatial launcher)
- Produksi: MediaPipe WASM task API (tanpa CDN) + HTTPS + deploy
