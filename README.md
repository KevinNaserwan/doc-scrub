# doc-scrub 🛡️

> **Zero-dependency, offline-first document & text privacy sanitizer for non-tech users, powered by Rust & WebAssembly.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.99.0-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Release-v1.0.0-blue.svg)](https://github.com/KevinNaserwan/doc-scrub)

---

## 📌 Mengapa DocScrub Dibuat? (Untuk Siapa Saja)
Setiap hari, staf HRD, tim legal, akuntan, guru, dan masyarakat umum harus membagikan dokumen atau tangkapan layar surat. Namun, sering kali dokumen tersebut mengandung data pribadi sensitif seperti **Nomor Induk Kependudukan (NIK/KTP)**, **alamat email pribadi**, **nomor WhatsApp/telepon**, atau **rekening bank**.

Layanan online seperti PDF converter umumnya mengunggah berkas ke server luar negeri, yang membahayakan privasi dan melanggar aturan perlindungan data pribadi (UU PDP / GDPR).

**`doc-scrub`** hadir sebagai solusi sederhana yang **bisa digunakan oleh siapa saja tanpa perlu mengerti koding**:
1. **100% Offline di Perangkat Pengguna:** Tidak ada satu byte pun data yang dikirim ke internet atau server mana pun.
2. **Otomatis Membersihkan Data Sensitif:** Cukup salin teks surat atau biodata, klik satu tombol, dan seluruh NIK, email, dan no HP otomatis disamarkan.

---

## ✨ Fitur Utama

* **⚡ Zero-Server / 100% Client-Side:** Seluruh proses scrubbing berjalan murni di peramban web (browser) pengguna menggunakan engine komputasi **Rust**.
* **🔍 Deteksi Pola Cerdas (Smart PII Redactor):**
  * NIK / KTP Indonesia (16 digit)
  * Alamat Email
  * Nomor Ponsel Indonesia (`08...`, `+62...`)
  * Nomor Kartu Kredit / Debit
* **🖥️ Tampilan Sederhana & Ramah Pengguna:** Dirancang bersih, modern, dan mudah dipahami oleh staf kantor maupun pengguna awam.

---

## 🚀 Cara Menggunakan

### Pengguna Umum (Non-Tech)
Cukup buka file `web/index.html` langsung di browser Anda (Google Chrome, Firefox, Safari, Microsoft Edge). Tidak memerlukan koneksi internet atau instalasi apa pun!

### Developer (Rust Core Library)
Bagi pengembang perangkat lunak yang ingin mengintegrasikan engine privasi Rust ini ke dalam aplikasi mereka:

```rust
use doc_scrub_core::DocScrubber;

fn main() {
    let scrubber = DocScrubber::new();
    let text = "Halo, nama saya Budi. Email: budi@ptba.co.id, No HP: 081299887766";
    let report = scrubber.sanitize(text);
    
    println!("Hasil Bersih: {}", report.sanitized_text);
    println!("Email disamarkan: {}", report.emails_masked);
}
```

---

## 📄 Lisensi
Proyek ini dilisensikan di bawah lisensi terbuka [MIT](LICENSE).

Dibuat dengan ❤️ oleh [Kevin Naserwan](https://github.com/KevinNaserwan).
