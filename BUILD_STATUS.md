# Spatial Vision — Status Build

**Tanggal:** 3 Aug 2026
**Lokasi:** `/Users/zaryu/Desktop/Niumination/sites/spatial-vision/`
**Mode:** Spatial Launcher — kursor+klik, swipe navigasi, self-host MediaPipe ✅

## Baseline Performa (3 Aug 2026)

**FPS terukur via kamera:** **58 FPS** (hampir cap 60Hz kamera).

| Konfigurasi | Sebelum | Sesudah |
|-------------|:-------:|:-------:|
| Model | `modelComplexity: 1` (full) | `0` (lite) |
| Tangan | `maxNumHands: 2` | `1` |
| Kamera | 960×540 | 640×360 |
| Respons | terasa lambat | ~real-time |

## Hasil Audit (30 Jul 2026)

- ✅ `frontend/src/lib.rs` — JSON output fixed (missing `}}`), added `init()`
- ✅ `frontend/src/gesture.rs` — **dihapus** (dead code, tidak di-compile — no `mod gesture` di lib.rs)
- ✅ `frontend/src/mediapipe.rs` — **dihapus 3 Aug** (dead code — no `mod mediapipe`)
- ✅ `frontend/index.html` — THREE.js CDN dihapus (tak terpakai)
- ✅ `backend/src/main.rs` — **disederhanakan**: hapus WebSocket broadcast (yagni, tanpa klien)
- ✅ `frontend/Cargo.toml` — **dipersingkat**: hapus 11 fitur `web-sys` + `serde` (tak dipakai)
- ✅ `backend/Cargo.toml` — hapus `serde` + `serde_json` (tak dipakai setelah WS dihapus)
- ✅ `Cargo.toml` (root workspace) — fix profile name: `frontend` → `spatial-vision-frontend`
- ✅ **Frontend WASM** — `cargo check --target wasm32-unknown-unknown` → ✅
- ✅ **Backend Axum** — `cargo check` → ✅

## Spatial Launcher (3 Aug 2026 — kursor + klik + swipe + self-host)

Tiga paket lanjutan dari roadmap dieksekusi (kode lengkap di sesi sebelumnya,
berhasil diverifikasi & di-commit di sesi ini):

| Paket | Implementasi | Status |
|-------|--------------|:------:|
| **Kursor + klik (spatial launcher)** | Kursor dari ujung telunjuk [8] (mirror x), smoothing 0.5; rising-edge pinch = klik pada kartu launcher (2 halaman × 4 kartu); hover highlight; feedback flash "✅ <nama>" | ✅ |
| **Swipe navigasi** | Window 150ms pada wrist [0], `|dx| > 0.35`, min span 60ms, cooldown 600ms → ganti halaman + dot indikator | ✅ |
| **Self-host MediaPipe** | `frontend/vendor/mediapipe/` (11MB, 7 file) — offline, tanpa CDN; `locateFile` relatif; ✅ **WAJIB di-track** (override global `vendor/` via `!frontend/vendor/` di .gitignore) | ✅ |

**Verifikasi (sesi ini):**

- ✅ Rust unit test 5/5 (pinch, palm, hysteresis, JSON invalid, empty hands)
- ✅ `cargo check` backend bersih
- ✅ Server lokal :3000 — index.html, style.css, `vendor/mediapipe/hands.js`,
  `hands_solution_simd_wasm_bin.wasm`, `pkg/*.js` + `pkg/*.wasm` semua HTTP 200
- ✅ `dist/` sinkron penuh dengan source (`diff -rq` bersih)
- ✅ **Uji browser (headless)**: halaman load, `Hands` dari `vendor/` sukses (tanpa
  teks error), module WASM jalan (dots launcher dibuat, page 0 `active`)
- ✅ **Uji fungsional navigasi**: klik dot 2 → page 1 aktif; klik dot 1 → page 0
  aktif. `setPage` (fungsi yang sama dipanggil swipe) bekerja dua arah
- ⏭️ Uji kamera nyata (kursor/swipe/klik) belum bisa dijalankan headless — perlu
  manusia + browser + kamera. Kandidat verifikasi berikutnya.

## Gesture Core Polish (3 Aug 2026, commit `ec7d5d8`)

- **Hysteresis ambang ganda** — pinch ON `< 0.045`, OFF `> 0.06`, anti-jitter
- **Gesture `palm`** — jarak wrist(0)–middle tip(12) `> 0.40`
- **FPS counter** — badge status rolling 1 detik
- **Unit test: 5/5 ✅** (pinch, palm, hysteresis, JSON invalid, empty hands)

## Ringkasan Audit (30 Jul 2026)

| Kategori | Sebelum | Sesudah | Hemat |
|----------|:-------:|:-------:|:-----:|
| File source | 4 | 3 | -1 file |
| Baris kode Rust | ~93 | ~42 | **-51 baris** |
| Dependensi Rust | 13 baris | 4 baris | **-9 baris** |
| CDN eksternal | 3 (MediaPipe + Camera + THREE) | 2 | -1 load |
| Kompilasi | ⚠️ gesture.rs dead code | ✅ Bersih | — |

## Cara Menjalankan

```bash
cd frontend
wasm-pack build --target web --out-dir dist/pkg
cp index.html style.css dist/
cd ../backend
cargo run
# Buka http://localhost:3000
```
