# Panduan & Spesifikasi Pengembangan Teknologi Spatial Vision Murah/Gratis (Full-Stack Rust)

## 1. Pengenalan Konsep & Sensor
[cite_start]Membuat teknologi *spatial computing* seperti Apple Vision sangat memungkinkan untuk dilakukan dengan budget yang sangat murah atau bahkan 100% gratis dari sisi *software*[cite: 1]. [cite_start]Teknologi utama seperti *hand tracking* (*pinch* & *tap*), *eye tracking*, *depth mapping*, dan *spatial UI* pada dasarnya merupakan kombinasi dari algoritma *computer vision* dan pemrosesan grafik 3D[cite: 2].

[cite_start]Pengembangan prototipe ini tidak membutuhkan sensor LiDAR mahal maupun *hardware* khusus[cite: 3]:
* [cite_start]**Hand Tracking & Pinch:** Memanfaatkan AI untuk mengonversi gambar tangan dari webcam biasa menjadi 21 titik koordinat 3D secara *real-time*[cite: 4]. [cite_start]Dari titik koordinat tersebut, gestur seperti *pinch* (mencubit) atau *tap* dapat dideteksi secara langsung tanpa sensor fisik tambahan[cite: 5].

---

## 2. Alternatif Open-Source & Setup Hardware

### [cite_start]Tabel Komponen Software Open-Source (Gratis) [cite: 6, 7]
| Fitur Apple Vision | Alternatif Open-Source / Gratis | Keterangan & Cara Kerja |
| :--- | :--- | :--- |
| **Hand Tracking & Pinch** | [cite_start]MediaPipe Hands (Google) [cite: 8] | [cite_start]Ringan dan berjalan mulus di CPU laptop/smartphone menggunakan webcam RGB standar[cite: 9]. |
| **Eye Tracking** | [cite_start]WebGazer.js / MediaPipe Face Mesh [cite: 10] | [cite_start]Memprediksi koordinat fokus mata di layar melalui input kamera depan[cite: 10]. |
| **Spatial UI / Interaktif 3D** | [cite_start]WebXR + Three.js / A-Frame [cite: 11] | [cite_start]Menampilkan objek atau jendela 3D melayang yang dapat dipindahkan dengan gestur di browser[cite: 12]. |
| **Depth Estimation** | [cite_start]Depth Anything v2 / MiDaS AI [cite: 13] | [cite_start]Mengubah rekaman video 2D biasa menjadi *depth map* (peta kedalaman 3D) berbasis AI tanpa sensor *depth* khusus[cite: 13]. |

### [cite_start]Opsi Hardware [cite: 15]
1. [cite_start]**Setup Rp0 (Desktop / Laptop Mode):** [cite: 15]
   * [cite_start]**Kebutuhan:** Laptop/PC + webcam standar[cite: 15].
   * [cite_start]**Konsep:** Webcam menangkap gerakan tangan di meja, lalu aplikasi menggerakkan kursor atau objek 3D pada layar monitor[cite: 16]. [cite_start]Mode ini cukup untuk menguji logika interaksi *spatial computing*[cite: 17].
2. [cite_start]**Setup Murah (DIY Wearable ~Rp50.000 - Rp150.000):** [cite: 18]
   * [cite_start]**Kebutuhan:** Smartphone (Android/iOS) + *headset* VR Box / Google Cardboard bekas[cite: 18].
   * [cite_start]**Konsep:** Masukkan smartphone ke dalam *headset* VR Box, aktifkan kamera belakang untuk efek *passthrough*, lalu jalankan aplikasi WebXR melalui browser HP[cite: 19].

---

## 3. Pilihan Stack Pemrograman

* [cite_start]**Web Stack Standard (JS):** Menggunakan HTML + JavaScript + MediaPipe Hands + Three.js[cite: 20]. [cite_start]Kelebihannya dapat langsung diuji di browser laptop maupun HP tanpa perlu instalasi perangkat lunak yang berat[cite: 21].
* [cite_start]**Python Stack:** Menggunakan Python + OpenCV + MediaPipe[cite: 22]. [cite_start]Sangat cocok untuk riset, pemrosesan logika gestur, dan *desktop automation* lokal[cite: 22, 23].
* [cite_start]**Full-Stack Rust Stack (High Performance & WASM):** Menggunakan Rust pada Frontend (WebAssembly) dan Backend (Axum/Actix) untuk performa tinggi, efisiensi memori, serta *type-safety*[cite: 25].

---

## 4. Alur Kerja Integrasi Full-Stack Rust WASM

1. [cite_start]**Input Video:** *MediaPipe JS Engine* memproses gambar dari kamera webcam[cite: 27].
2. [cite_start]**Interop JS to WASM:** Koordinat 21 titik tangan dikirim ke *Rust WebAssembly Module* melalui `wasm-bindgen`[cite: 28].
3. [cite_start]**Pemrosesan Gestur:** *Rust WASM* mengeksekusi kalkulasi matematika gestur (*pinch*, *grab*, *drag*) secara efisien dan cepat[cite: 29].
4. [cite_start]**Rendering 3D:** *Rust WASM* memperbarui transformasi matriks 3D pada objek di kanvas WebGL/Three.js[cite: 30].
5. [cite_start]**Backend Server:** Backend Axum (Rust) bertugas melayani *asset compilation* serta menangani komunikasi WebSocket antar-klien untuk fitur *multiplayer / collaborative spatial workspace*[cite: 31].

---

## 5. Master Prompt: Full-Stack Rust Spatial Computing Web App

[cite_start]Gunakan prompt di bawah ini untuk dikirimkan ke AI (seperti ChatGPT/Claude) atau pengembang software guna membangun prototipe berbasis Rust[cite: 25].

```markdown
### SYSTEM PROMPT & SPECIFICATION: Full-Stack Rust Spatial Computing Web Application

#### 1. Peran & Tujuan
Bertindaklah sebagai Senior Full-Stack Rust Engineer dan Computer Vision Architect. Tugasmu adalah merancang dan menyediakan kode boilerplate lengkap untuk aplikasi web interaktif "Spatial Computing / Vision Pro Clone" dengan budget gratis. 

Aplikasi ini mendeteksi gestur tangan (hand tracking & pinch) dari kamera webcam secara real-time dan merender objek/antarmuka 3D melayang di browser.

---

#### 2. Arsitektur & Tech Stack

Seluruh aplikasi harus dibangun berbasis bahasa Rust baik untuk Backend maupun Frontend:

* **Frontend:**
  * **Framework:** Rust dengan Leptos atau Yew (dikompilasi ke WebAssembly / WASM).
  * **Computer Vision Interop:** Integrasi JS Interop via `wasm-bindgen` ke library MediaPipe Hands untuk hand tracking dan pinch gesture detection (21 titik koordinat 3D).
  * **Spatial 3D Engine:** WebGL / Three.js interop via WASM atau `three-rs` / `wgpu` untuk merender antarmuka 3D melayang (Spatial UI).
* **Backend:**
  * **Framework:** Rust (`Axum` atau `Actix-web`).
  * **Fitur Backend:** Menyajikan static files WASM/JS, kelola koneksi WebSocket real-time untuk sinkronisasi state multi-user, dan pengolahan data telemetri gestur.

---

#### 3. Fitur Utama yang Harus Diimplementasikan

1. **Video Passthrough & Capture (Frontend):**
   * Mengakses kamera webcam lokal (RGB) pengguna secara real-time di browser.
2. **Hand Tracking & Gesture Logic (Rust WASM + MediaPipe):**
   * Mengirimkan frame video ke MediaPipe Hands.
   * Menerima 21 landmark point 3D dalam WebAssembly.
   * Algoritma deteksi gestur pinch (jarak Euclidean antara ujung ibu jari dan ujung jari telunjuk < threshold tertentu) yang ditulis dalam Rust.
3. **Spatial UI & Interaksi 3D (WebGL / Three.js via WASM):**
   * Merender kanvas 3D melayang (cube atau window panel).
   * Menghubungkan koordinat tangan dengan posisi kursor/objek 3D.
   * Ketika gestur pinch terdeteksi, pengguna dapat menggenggam, memindahkan, dan melepaskan objek 3D di dalam ruang virtual.
4. **Backend Server (Axum):**
   * Menjalankan server HTTP & WebSocket berbasis async/await Rust (`tokio`).

---

#### 4. Output Kode & Struktur Project yang Diharapkan

Tolong sediakan:
1. `Cargo.toml` untuk Workspace/Frontend/Backend dengan dependency yang dibutuhkan (`wasm-bindgen`, `web-sys`, `axum`, `tokio`, `serde`, dll).
2. Struktur File Proyek:
   ├── backend/
   │   └── src/main.rs (Axum Web Server & WebSockets)
   ├── frontend/
   │   ├── src/
   │   │   ├── lib.rs (Logic Utama Leptos/Yew & WASM Binding)
   │   │   ├── gesture.rs (Logika Deteksi Pinch dalam Rust)
   │   │   └── render.rs (Canvas 3D / Three.js Interop)
   │   └── index.html
3. Kode `frontend/src/gesture.rs`: Fungsi Rust murni untuk memproses koordinat 3D dari MediaPipe dan menghitung jarak pinch.
4. Kode `frontend/src/lib.rs`: Integrasi kamera, pengiriman frame ke WASM, dan pembaruan kanvas 3D.
5. Kode `backend/src/main.rs`: Server Axum sederhana untuk serving aplikasi WASM.
6. Panduan Build & Run: Langkah-langkah kompilasi menggunakan `wasm-pack` atau `trunk`.