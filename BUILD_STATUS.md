# Spatial Vision — Status Build

**Tanggal:** 5 Aug 2026
**Lokasi:** `/Users/zaryu/Desktop/Niumination/sites/spatial-vision/`
**Mode:** Spatial Launcher — kursor+klik, swipe navigasi, self-host MediaPipe ✅

## Audit Browser Headless (5 Aug 2026)

**Uji nyata via Playwright Chromium** — 2 konteks: (A) fake camera → graph
MediaPipe + FPS loop; (B) kamera ditolak → seluruh interaksi mouse Level 2.

**Hasil:** **22/22 ✅** — 4 bug nyata ditemukan & diperbaiki:

| # | Bug | Perbaikan |
|---|-----|-----------|
| 1 | MediaPipe gagal init — `hand_landmark_lite/full.tflite` hilang dari vendor (`model identifier 'd', should be 'TFL3'`) | Tambah kedua tflite dari CDN resmi `@mediapipe/hands@0.4.1675469240` (magic bytes `TFL3` ✅) |
| 2 | Dots navigasi tidak bisa diklik — `#dots` mewarisi `pointer-events:none` dari `#launcher` | `pointer-events:auto` pada `#dots` |
| 3 | Dwell-to-select mouse mati — `updateHover` hanya dipanggil dari loop kamera | Listener `mousemove` level document → `updateHover(..., 'mouse')` |
| 4 | Frame tanpa tangan membatalkan dwell mouse | State `dwellSource` — hanya dwell sumber tangan yang dibatalkan saat tangan hilang |

**Juga:** `.catch()` pada `camera.start()` → badge "Kamera tidak tersedia
(izin ditolak)" saat izin ditolak; interaksi mouse tetap jalan penuh.

**Cakupan uji konteks B (16 asersi):** buka modal, judul/deskripsi/href, tutup
modal, navigasi dots dua arah, dwell 900ms buka kartu benar, dwell <900ms
dibatalkan, link Buka → tab baru, nol error JS.

**Sisa (butuh kamera fisik):** gestur nyata — kursor tangan, pinch=klik,
swipe, palm=tutup (Level 3 TESTING.md).

## Audit Browser Headless — Lanjutan dari Sesi `bear` (5 Aug 2026)

Sesi `bear` (4 Aug, 17:09 → 18:28) terputus saat debugging swipe + deteksi
jari setelah audit headless awal (commit `6fde043`). User melaporkan:
"jari hanya 1 dengan lingkaran hijau, swipe tidak berfungsi".

### Bug utama yang ditemukan & diperbaiki

| # | Bug | Akar | Perbaikan (dalam sesi ini) |
|---|-----|------|---------------------------|
| 5 | **Kerangka tangan tidak tergambar** — hanya lingkaran hijau kursor terlihat (`0 px`) | `smoothHand()` mencampur objek `{x,y,z}` (MediaPipe) dengan indeks array `[0]` → semua landmark jadi NaN; `drawHand()` menerima NaN | Seragamkan ke objek `{x,y,z}`: smoothing per properti `.x/.y/.z`, `drawHand` akses `lm[i].x` |
| 6 | **Swipe tidak berfungsi** — threshold 0.35/150ms pakai wrist [0] terlalu ketat dan arah mungkin salah | Wrist bergerak sedikit saat swipe; konvensi arah belum konsisten dengan TESTING.md (`kiri layar = halaman berikutnya` di mirror `scaleX(-1)`) | Rompak swipe: ujung telunjuk [8], window **220ms**, threshold `|dx| > 0.22`, rasio kelurusan `> 0.45`, cooldown 500ms, arah dibalik (`dx < 0` = next) |
| 7 | **Loop kamera mati saat landmark NaN** — `elementFromPoint(NaN)` melempar `DOMException` | MediaPipe kadang output NaN saat gerak cepat/awal init; error tidak ditangkap | Guard `if (!Number.isFinite(cursor.x/y)) return null` di `elementAtCursor()`; guard non-finite di `onResults`; `.catch()` di `camera.start()` dan `hands.send()` |
| 8 | **Swipe memicu dwell modal tak sengaja** — kursor langsung hover kartu setelah swipe selesai | Tidak ada "cooldown hover" setelah swipe | `swipeMuteUntil = now + 450ms` + hapus `hover` saat `src === 'hand'` dalam mute window |

### Verifikasi sesi ini (5 Aug 2026)

- ✅ Rust unit test 5/5 (`pinch`, `palm`, `hysteresis`, `JSON invalid`, `empty hands`)
- ✅ `cargo check` backend & WASM `wasm-pack build` bersih
- ✅ `frontend/index.html` sintaks JS valid; `dist/index.html` sinkron ulang
- ✅ **Headless smoke test (2 konteks Playwright):**
  - Badge `Palm · FPS 1` (pipeline MediaPipe hidup dengan fake camera y4m)
  - **MAX PX: 6202** (kerangka tangan lengkap 21 landmark + tulang tergambar — sebelumnya 0 px sebelum fix NaN)
  - Tanpa error JS (`errors.length === 0`)
  - **Swipe headless: SKIP** — `FPS 1` headless (tanpa GPU SwiftShader) tidak mencapai `>= 8` FPS yang dibutuhkan window 220ms + 4 sampel; ini **batas lingkungan**, bukan bug app (dokumentasi di TESTING.md 3.3)
- ✅ **Screenshot verifikasi (`brain/projects/spatial-vision/skeleton-fixed.png`):** kerangka hijau
  (tulang) + titik biru (landmark) terlihat jelas di layar, badge `Palm · FPS 1`
  di kanan atas — bukti visual pipeline lengkap bekerja.

### Catatan produksi

- **Swipe harus diuji dengan kamera fisik** (headless tidak cukup cepat untuk
  window 220ms/4 sample). Uji manual: ayun tangan cepat kiri/kanan di depan
  kamera, pastikan halaman berpindah (`page` 0↔1) dan dot indikator berubah.
- **Kerangka tangan lengkap sekarang terlihat** — bukan cuma lingkaran hijau
  kursor. Ini menjawab keluhan user "hanya 1 jari terlihat dengan lingkaran hijau":
  semua 21 landmark (ibu jari, telunjuk, tengah, manis, kelingking) + tulang
  (hijau) sekarang digambar (`drawHand`).
- Perbaikan `smoothHand` + `drawHand` juga mempengaruhi tampilan kursor —
  smoothing 0.6 (kursor) + smoothing 0.45 (kerangka) membuat tampilan lebih
  halus daripada sebelumnya.

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

## App Modal + Dwell-to-Select (3 Aug 2026)

Kartu launcher jadi app sungguhan (Next Steps #2–#3 AGENTS.md) — 3 paket:

| Paket | Implementasi | Status |
|-------|--------------|:------:|
| **README update** | Deskripsi self-host MediaPipe (bukan CDN), tabel fitur launcher, catatan produksi | ✅ |
| **Kartu → app modal** | `data-href` + `data-desc` di tiap kartu; klik → modal (judul, deskripsi, tombol "Buka <app>" `target=_blank`, "Tutup"); palm membuka telapak = tutup modal | ✅ |
| **Dwell-to-select** | Tahan kursor di kartu 900ms (`DWELL_MS`) = pilih; progress bar hijau via CSS `animation: dwell .9s`; timer dibatalkan saat tangan hilang/modal terbuka/kursor pindah; dual input mouse (klik langsung) sekalian testable | ✅ |

**Verifikasi (browser live :3000):**

- ✅ Page load — 8 kartu (2 halaman × 4) render, tanpa error
- ✅ Klik kartu Media → modal terbuka: judul "Media", deskripsi "Putar video di
  YouTube", link `https://www.youtube.com` (text "Buka Media"), tombol "Tutup"
- ✅ Klik "Tutup" → modal tertutup (DOM `#modal` kembali `hidden`)
- ✅ `dist/` disinkron ulang dari source (index.html + style.css identik)
- ⏭️ Dwell-to-select & palm-close jalan via jalur yang sama (`activateCard` /
  `closeApp` terverifikasi; timer 900ms = `setTimeout` → `activateCard`) — uji
  penuh butuh kamera nyata (gestur tangan asli)

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
