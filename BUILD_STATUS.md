# Spatial Vision — Status Build

**Tanggal:** 22 Jul 2026  
**Lokasi:** `/Users/zaryu/Desktop/Niumination/projects/spatial-vision/`  
**Mode:** Build (from Plan) — DOX Protocol ✅

## Hasil

- ✅ File proyek disalin dari zip (`spatial-vision-arena.zip`)
- ✅ Struktur workspace Rust (`Cargo.toml` workspace)
- ✅ Frontend WASM dikompilasi (`spatial_vision_frontend.wasm` — 794KB)
- ✅ `frontend/dist/` dibuat dengan `index.html`, `style.css`, `pkg/`, JS loader
- ✅ Backend (`backend/src/main.rs`) siap (Axum 0.7, tokio, WebSocket `/ws`)
- ✅ Dokumentasi (`README.md`, `uploads/Spatial-Vision Full Stack Rust.md`)

## Struktur File

```
spatial-vision/
├── Cargo.toml              # Workspace
├── README.md               # Dokumentasi proyek
├── BUILD_STATUS.md         # File ini
├── backend/
│   ├── Cargo.toml
│   └── src/main.rs         # Axum server + WebSocket broadcast
├── frontend/
│   ├── Cargo.toml          # wasm-bindgen + web-sys
│   ├── index.html
│   ├── style.css
│   ├── src/lib.rs          # WASM entry (kamera + canvas)
│   �── src/gesture.rs      # Logika pinch (MediaPipe 21 landmark)
│   └── src/render.rs       # Render canvas 2D
│   └── dist/               # Build output (WASM + HTML + CSS)
│       ├── index.html
│       ├── style.css
│       ├── pkg/
│       │   ├── spatial_vision_frontend_bg.wasm
│       │   �── spatial_vision_frontend.js
│       └── spatial_vision_frontend.js
└── target/
    └── wasm32-unknown-unknown/release/
        └── spatial_vision_frontend.wasm
```

## Cara Menjalankan

```bash
# 1. Build WASM (sudah selesai)
cd frontend
cargo build --target wasm32-unknown-unknown --release

# 2. Jalankan backend server
cd ../backend
cargo run  # Server di http://localhost:3000
```

## Catatan Implementasi

- Frontend menggunakan `wasm-bindgen` (WebAssembly) untuk akses kamera (`getUserMedia`) dan render canvas (`CanvasRenderingContext2d`).
- Modul `gesture.rs` berisi deteksi `pinch` berdasarkan jarak Euclidean antara landmark MediaPipe (ibu jari = 4, telunjuk = 8).
- Backend `main.rs` melayani file statis (`ServeDir`) dan menyediakan `/ws` untuk broadcast JSON antar klien (multiplayer/collaborative workspace).
- Untuk produksi, tambahkan pipeline MediaPipe JS di `index.html` dan kirim 21 landmark ke WASM melalui `wasm-bindgen`.

## Referensi Dokumentasi Lengkap

Lihat `uploads/Spatial-Vision Full Stack Rust.md` (panduan master prompt, tabel komponen open-source, setup hardware, alur integrasi full-stack Rust WASM).
