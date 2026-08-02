# Spatial Vision — Project AGENTS.md

**Lokasi:** `sites/spatial-vision/`
**Stack:** Rust (Axum backend) + Rust/WASM (frontend), MediaPipe Hands via CDN
**Remote:** `github.com/Niumination/spatial-vision`
**Deploy:** ⚪ Prototype — lokal port 3000
**Status:** 🆕 Active — 2 Aug 2026

## Overview

Deteksi gestur pinch tangan via kamera. Backend Axum serve statis, frontend
WASM (Rust) hitung jarak 3D ibu jari–telunjuk dari landmark MediaPipe.

## Struktur

```
spatial-vision/
├── backend/src/main.rs      Axum — serve frontend/dist/ di :3000
├── frontend/
│   ├── src/lib.rs           detect_pinch + parse_pinch + unit test (4)
│   ├── src/mediapipe.rs     process_pinch_data — BELUM terpakai (candidate hapus)
│   ├── index.html           MediaPipe pipeline + kamera
│   └── style.css
└── uploads/                 Draft spesifikasi
```

## Build & Test

```bash
cd frontend && wasm-pack build --target web --out-dir dist/pkg
cp index.html style.css dist/
cd ../backend && cargo run          # → http://localhost:3000
cargo test --manifest-path frontend/Cargo.toml
```

## Aturan Proyek

1. Logika gesture = murni di klien; kamera tidak pernah ke backend
2. Jangan tambah WebSocket/server push tanpa klien yang jelas (yagni — sudah pernah dihapus)
3. `parse_pinch` (logika murni) wajib di-test; wrapper wasm `detect_pinch` cukup tipis
4. Jika `mediapipe.rs` tetap tak terpakai → hapus (dead code)
5. Update README setiap ada perubahan arsitektur
