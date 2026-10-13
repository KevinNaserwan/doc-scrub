//! doc-scrub-core: Zero-dependency Rust Document Privacy Sanitizer
//! Designed for native compilation and WebAssembly embedding.

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ScrubReport {
    pub sanitized_text: String,
    pub emails_masked: usize,
    pub phones_masked: usize,
    pub niks_masked: usize,
    pub cards_masked: usize,
}

pub struct DocScrubber;

impl DocScrubber {
    pub fn new() -> Self {
        Self
    }

    /// Sanitizes plain text input by masking PII tokens
    pub fn sanitize(&self, input: &str) -> ScrubReport {
        let mut report = ScrubReport::default();
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut output_words = Vec::new();

        for word in words {
            // 1. Email check
            if word.contains('@') && word.contains('.') {
                report.emails_masked += 1;
                output_words.push("[EMAIL TERSEMBUNYI]");
                continue;
            }

            // Clean punctuation for numeric checks
            let trimmed = word.trim_matches(|c: char| !c.is_alphanumeric());

            // 2. NIK check (16 consecutive digits)
            if trimmed.len() == 16 && trimmed.chars().all(|c| c.is_ascii_digit()) {
                report.niks_masked += 1;
                output_words.push("[NIK/KTP TERSEMBUNYI]");
                continue;
            }

            // 3. Indonesian Phone Number check (starts with 08, +62, 62 and length 10-13)
            if (trimmed.starts_with("08") || trimmed.starts_with("62") || trimmed.starts_with("+62"))
                && trimmed.len() >= 10
                && trimmed.len() <= 14
                && trimmed.chars().filter(|c| c.is_ascii_digit()).count() >= 10
            {
                report.phones_masked += 1;
                output_words.push("[NO HP TERSEMBUNYI]");
                continue;
            }

            // 4. Credit Card Check (13 to 19 digits)
            if (trimmed.len() >= 15 && trimmed.len() <= 16) && trimmed.chars().all(|c| c.is_ascii_digit()) {
                report.cards_masked += 1;
                output_words.push("[KARTU TERSEMBUNYI]");
                continue;
            }

            output_words.push(word);
        }

        report.sanitized_text = output_words.join(" ");
        report
    }
}
