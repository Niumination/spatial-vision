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
│   ├── src/lib.rs           detect_pinch (wasm wrapper) + parse_pinch (logika) + 4 unit test
│   ├── src/mediapipe.rs     process_pinch_data — ❌ DEAD CODE (candidate hapus)
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
4. **`mediapipe.rs` dead code** — jika tetap tak terpakai, hapus.
5. **Repo hygiene — jangan commit artefak build**: `frontend/dist/`, `target/`,
   `*.wasm` di-ignore. Clone harus bisa rebuild via `wasm-pack` (glue tanpa wasm
   = app 404). Build dulu → commit hanya source.
6. **Backend path** — `ServeDir` relatif ke CWD; dokumentasikan menjalankan dari
   `backend/` agar tidak 404 (`../frontend/dist`).
7. Update README & BUILD_STATUS setiap ada perubahan arsitektur.

## Input/Output WASM

- **Input `detect_pinch`:** array tangan dari `multiHandLandmarks` —
  `[[[x,y,z]x21], [[x,y,z]x21]]` (normalized, bisa multi-tangan; ambil jarak min).
- **Output JSON:** `{"isPinching":bool,"strength":float}`.
- Threshold pinch: jarak 3D `thumb(4)–index(8) < 0.05`; `strength` memudar
  linear hingga 0.15.

## Catatan Perubahan

| Tanggal | Commit | Isi |
|---------|--------|-----|
| 3 Aug 2026 | `f763d79` | index.html rapi + error handling MediaPipe CDN + import `detect_pinch` langsung dari glue |
| 2 Aug 2026 | `43634a9` | pinch detection 3D + test, static path fix, **untrack dist** (repo hygiene) |
| 2 Aug 2026 | `71635b4` | Ponytail audit — hapus dead code, sederhanakan backend/frontend (-51 baris Rust, -9 deps) |
| 22 Jul 2026 | `03f329d` | gesture recognition + integrasi MediaPipe |
| 22 Jul 2026 | `19e8f76` | initial commit — full-stack Rust/WASM |

## Next Steps (kandidat)

- Hapus `mediapipe.rs` bila masih yatim
- Produksi: pindah MediaPipe ke WASM task API (tanpa CDN) + HTTPS + deploy
- Kalibrasi threshold pinch per-jarak kamera
