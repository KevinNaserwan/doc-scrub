# doc-scrub 🛡️

> **Zero-dependency, offline-first document & text privacy sanitizer for non-tech users, powered by Rust & WebAssembly.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.99.0-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Release-v1.0.0-blue.svg)](https://github.com/KevinNaserwan/doc-scrub)

---

## 📌 Why DocScrub? (Built for Everyone)
Every day, HR staff, legal teams, accountants, educators, and everyday people need to share documents, scanned letters, and screenshots. However, these documents often contain sensitive Personally Identifiable Information (**PII**) such as **National Identification Numbers (NIK/SSN)**, **personal email addresses**, **phone/WhatsApp numbers**, or **credit card details**.

Most free online document converters and text utilities upload your files to remote cloud servers, putting privacy at risk and potentially violating strict data protection regulations (**GDPR, HIPAA, and UU PDP**).

**`doc-scrub`** provides an intuitive, foolproof solution that **anyone can use without any technical or coding knowledge**:
1. **100% Offline & Client-Side:** Not a single byte of your data ever leaves your device or touches an external server.
2. **Automated Sensitive Data Masking:** Simply paste your document text or letter, click one button, and sensitive IDs, emails, and phone numbers are redacted instantly.

---

## ✨ Key Features

* **⚡ Zero-Server Architecture:** All sanitization computation executes strictly inside your local web browser, powered by a high-performance **Rust** engine.
* **🔍 Intelligent Pattern Detection (Smart PII Redactor):**
  * National ID Numbers (16-digit format / NIK)
  * Email Addresses
  * Mobile Phone Numbers (`+62...`, `08...`, international formats)
  * Credit & Debit Card Numbers
* **🖥️ Non-Tech Friendly Interface:** A clean, modern, distraction-free interface designed for office workers and general users.
* **🔒 Peace of Mind:** Completely safe to use even when your device is fully disconnected from the internet.

---

## 🚀 How to Use

### For General Users (Non-Tech)
Simply open [`web/index.html`](web/index.html) directly in any modern web browser (Google Chrome, Mozilla Firefox, Safari, Microsoft Edge). No command line, installation, or internet connection required!

1. Paste or type your raw document text into the left panel.
2. The PII detector will automatically identify and mask sensitive tokens in real time.
3. Click **"Copy Result"** to copy the sanitized text directly to your clipboard.

### For Developers (Rust Core Engine)
Software engineers can easily embed the zero-dependency Rust core library into their backend services or desktop applications:

```rust
use doc_scrub_core::DocScrubber;

fn main() {
    let scrubber = DocScrubber::new();
    let text = "Hello, my name is Alex. Email: alex@example.com, Phone: 081299887766, ID: 3271012345678901";
    let report = scrubber.sanitize(text);
    
    println!("Sanitized Text: {}", report.sanitized_text);
    println!("Emails Masked: {}", report.emails_masked);
    println!("Phones Masked: {}", report.phones_masked);
    println!("IDs Masked: {}", report.niks_masked);
}
```

---

## 🧪 Testing & Verification

Run the Rust test suite:
```bash
cargo test
```

Verify WebAssembly target compilation:
```bash
cargo check --target wasm32-unknown-unknown
```

---

## 📄 License
This project is open-source and licensed under the [MIT License](LICENSE).

Authored with ❤️ by [Kevin Naserwan](https://github.com/KevinNaserwan).
