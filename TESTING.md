# Spatial Vision — Panduan Uji Coba

**Target:** Launcher spatial berbasis kamera (kursor tangan, pinch = klik, palm =
tutup, swipe = pindah halaman, dwell 900ms = pilih kartu).

Uji dibagi 3 level: **otomatis → tanpa kamera (mouse) → kamera nyata**. Kerjakan
berurutan; setiap level adalah prasyarat level berikutnya.

---

## Level 0 — Persiapan

```bash
cd ~/Desktop/Niumination/sites/spatial-vision

# Prasyarat (sekali)
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# Build frontend
cd frontend
wasm-pack build --target web --out-dir dist/pkg
cp index.html style.css dist/
cp -R vendor dist/

# Jalankan backend (WAJIB dari backend/, path serve relatif ke CWD)
cd ../backend
cargo run
```

Buka `http://localhost:3000` di **Chrome/Edge** (kamera hanya jalan di
localhost/HTTPS). Grant permission kamera saat browser minta.

**Cek cepat server sehat** (terminal kedua):

```bash
curl -I http://localhost:3000/                      # → 200
curl -I http://localhost:3000/pkg/spatial_vision_frontend_bg.wasm  # → 200
curl -I http://localhost:3000/vendor/mediapipe/hands.js            # → 200
```

> Glue WASM & vendor paling rawan 404 — kalau salah satu bukan 200, app rusak
> meski index.html kebuka.

---

## Level 1 — Uji Otomatis (tanpa browser)

```bash
cargo test --manifest-path frontend/Cargo.toml
```

**Harus lulus 5/5:** pinch dekat, palm terbuka, hysteresis deadband, JSON
invalid, tanpa tangan. Ini jaring pengaman logika gesture di Rust. Kalau gagal,
jangan lanjut ke level berikutnya.

---

## Level 2 — Uji Tanpa Kamera (dual input mouse)

Tidak perlu kamera — seluruh interaksi kartu bisa diuji dengan mouse. Ini
memverifikasi launcher, modal, dan dwell **sebelum** gestur tangan.

| # | Langkah | Hasil yang diharapkan |
|---|---------|----------------------|
| 1 | Buka `http://localhost:3000` | Badge status "Menunggu kamera..." lalu "Waiting · FPS 0" (kamera belum ada / denied) |
| 2 | Perhatikan launcher | 2 halaman × 4 kartu = 8 kartu, dot indikator 2, dot pertama `active` |
| 3 | Klik kartu **Media** | Modal terbuka: judul "Media", deskripsi "Putar video di YouTube", tombol "Buka Media" + "Tutup" |
| 4 | Klik **Tutup** | Modal tertutup (DOM `#modal` `hidden`) |
| 5 | Klik dot 2 | Pindah ke halaman 1 (kartu Settings/News/Mail/Games), dot 2 `active` |
| 6 | Klik dot 1 | Kembali ke halaman 0 |
| 7 | Hover kartu **Maps** 900ms **tanpa klik** | Progress bar hijau jalan (animasi dwell), lalu kartu aktif + modal terbuka otomatis |
| 8 | Tutup modal, hover kartu lain lalu pindah kursor < 900ms | Tidak ada modal — dwell dibatalkan saat kursor pindah |
| 9 | Klik **Buka Maps** di modal | Tab baru terbuka ke `https://maps.google.com` |

**Catatan:** langkah 7–8 menguji jalur `dwellTimer` → `activateCard` yang sama
dipakai gestur dwell-to-select. Kalau lulus, bagian dwell di kamera tinggal
membuktikan kursor stabil di atas kartu.

---

## Level 3 — Uji Kamera Nyata (gestur tangan)

### Kondisi lingkungan (penting!)

- Ruangan **terang** (MediaPipe lite model butuh cahaya cukup)
- Latar belakang relatif polos, hindari barang mirip tangan di belakang
- Tangan **30–70 cm** dari kamera, menghadap lensa
- 1 tangan dulu (maxNumHands = 1)

### 3.1 Status & FPS

| # | Langkah | Hasil |
|---|---------|-------|
| 1 | Angkat tangan di depan kamera | Badge berubah "Waiting · FPS ~50-60" (bukan "Menunggu kamera...") |
| 2 | Tutup tangan jadi kepalan lalu buka | Kursor hijau muncul mengikuti ujung telunjuk |
| 3 | Gerakkan tangan pelan kiri-kanan-atas-bawah | Kursor mengikuti dengan smoothing (tidak melompat), posisi x ter-mirror |
| 4 | Catat FPS di badge | Target ≥ 30 (lite model + 640×360). Di bawah 20 → lihat Troubleshooting |

### 3.2 Pinch = klik

| # | Langkah | Hasil |
|---|---------|-------|
| 1 | Arahkan kursor ke kartu **Media** (hover → kartu highlight) | Kartu punya class `hover` |
| 2 | Rapatkan ibu jari + telunjuk (pinch), tahan | Badge `Pinch! 0.0x`, ring luar hijau di kursor, kartu terbuka (modal) |
| 3 | Buka pinchnya | Modal tetap terbuka |
| 4 | Tutup modal dengan palm atau klik **Tutup**, ulangi di kartu lain | Semua kartu membuka modal yang benar |

**Kalibrasi pinch:** threshold ON `< 0.045`, OFF `> 0.06` (hysteresis). Kalau
terlalu sensitif (klik tidak sengaja) atau terlalu berat (sulit klik), nilai ini
di `frontend/src/lib.rs` / `parse_pinch`.

### 3.3 Swipe = pindah halaman

| # | Langkah | Hasil |
|---|---------|-------|
| 1 | Pastikan di halaman 0 (dot 1 active) | — |
| 2 | Ayunkan pergelangan cepat ke **kiri** (tangan gerak ke kiri, ~150ms) | Pindah ke halaman 1 (dot 2 active) |
| 3 | Ayunkan ke **kanan** | Kembali ke halaman 0 |
| 4 | Gerakkan tangan pelan (tanpa ayunan cepat) | **Tidak** pindah halaman — swipe butuh `|dx| > 0.35` dalam 150ms |

**Kalibrasi:** threshold di `frontend/index.html` (`dx > 0.35`, cooldown 600ms).

### 3.4 Dwell-to-select (alternatif klik)

| # | Langkah | Hasil |
|---|---------|-------|
| 1 | Arahkan kursor ke kartu, **tahan diam** 900ms | Progress bar hijau penuh → modal terbuka otomatis |
| 2 | Hover lalu gerakkan kursor sebelum 900ms | Tidak ada modal (dwell dibatalkan) |
| 3 | Lepaskan tangan dari frame saat dwell berjalan | Tidak ada modal (dibatalkan saat tangan hilang) |

### 3.5 Palm = tutup modal

| # | Langkah | Hasil |
|---|---------|-------|
| 1 | Buka modal (klik/pinch/dwell) | Modal tampil |
| 2 | Buka telapak penuh di depan kamera (jari renggang) | Badge `Palm`, modal tertutup |
| 3 | Ulangi 2–3× | Konsisten tertutup tiap palm |

**Threshold palm:** jarak wrist(0)–middle tip(12) `> 0.40` dan tidak pinch.

### 3.6 Edge case

| Kasus | Hasil yang diharapkan |
|-------|----------------------|
| Tangan keluar frame saat pinch | Kursor hilang, status reset (badge "Waiting"), tidak ada klik menggantung |
| Pinch di tengah (bukan di kartu) | Tidak ada klik — hanya kursor |
| Dua tangan masuk frame | Hanya tangan pertama (index 0) yang dipakai — stabil, tidak kacau |
| Kamera ditutup tangan / gelap | Badge "Waiting", kursor hilang — tidak crash |

---

## Checklist Akhir

- [ ] Level 1: unit test 5/5 ✅
- [ ] Level 2: modal buka/tutup, navigasi dots, dwell mouse, link Buka ✅
- [ ] Level 3: kursor mengikuti, FPS ≥ 30
- [ ] Pinch membuka modal (semua kartu)
- [ ] Swipe dua arah (kiri/kanan), gerak pelan tidak pindah
- [ ] Dwell 900ms memilih kartu; < 900ms dibatalkan
- [ ] Palm menutup modal
- [ ] Tangan keluar frame = reset bersih tanpa klik menggantung

---

## Troubleshooting

| Gejala | Penyebab | Solusi |
|--------|----------|--------|
| Badge "Error: MediaPipe gagal dimuat" | `vendor/` tidak tersaji | `cp -R vendor dist/` lalu refresh; cek `curl /vendor/mediapipe/hands.js` → 200 |
| Halaman 404 / wasm 404 | Jalankan dari folder salah | `cd backend && cargo run` (ServeDir relatif ke CWD) |
| Kamera tidak muncul / denied | Bukan localhost/HTTPS, atau permission ditolak | Buka `http://localhost:3000`; reset permission kamera di browser |
| Tangan tidak terdeteksi | Cahaya kurang / jarak jauh | Perbesar cahaya, dekatkan tangan 30–70cm |
| FPS rendah (< 20) | Model lite belum aktif / terlalu banyak aplikasi | Cek `modelComplexity: 0` di index.html; tutup tab berat; coba Chrome |
| Pinch terlalu sensitif/berat | Threshold `0.045/0.06` belum cocok dengan jarak duduk | Sesuaikan di `frontend/src/lib.rs` → `parse_pinch`; rebuild wasm + `cp pkg dist/` |
| Swipe tidak terdeteksi | Ayunan terlalu pelan | Ayun lebih cepat & lebar (`|dx| > 0.35` dalam 150ms) |
| Perubahan kode tidak muncul | `dist/` tidak di-rebuild | `wasm-pack build` ulang + `cp index.html style.css dist/` + hard refresh (Cmd+Shift+R) |

---

## Referensi Threshold (untuk kalibrasi)

| Parameter | Nilai | Lokasi |
|-----------|-------|--------|
| Pinch ON / OFF | `< 0.045` / `> 0.06` | `frontend/src/lib.rs` (`parse_pinch`) |
| Palm wrist–middle tip | `> 0.40` | `frontend/src/lib.rs` |
| `strength` fade-out | linear sampai jarak `0.15` | `frontend/src/lib.rs` |
| Swipe | `|dx| > 0.35`, window 150ms, cooldown 600ms | `frontend/index.html` |
| Dwell | `900ms` (`DWELL_MS`) | `frontend/index.html` |
| Smoothing kursor | `0.5` | `frontend/index.html` |
| Kamera | 640×360, lite model, 1 hand | `frontend/index.html` |
