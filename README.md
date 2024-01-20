# Stego WPS

[![Crates.io](https://img.shields.io/crates/v/stego_wps)](https://crates.io/crates/stego_wps)
[![Downloads](https://img.shields.io/crates/d/stego_wps.svg)](https://crates.io/crates/stego_wps)
[![Documentation](https://docs.rs/stego_wps/badge.svg)](https://docs.rs/stego_wps)
[![License](https://img.shields.io/crates/l/stego_wps)](https://crates.io/crates/stego_wps)
[![Dependency Status](https://deps.rs/repo/github/JamesClarke7283/stego_wps/status.svg)](https://deps.rs/repo/github/JamesClarke7283/stego_wps)

## About
Stego WPS is a Rust library designed for text-based steganography using the Words Per Sentence (WPS) method. It enables the encoding and decoding of hidden messages within the structure of a given text, offering a unique approach to conceal information in plain sight.

## Features
- **Encoding and Decoding**: Hide and retrieve messages using the WPS method.
- **ASCII Compliance**: Ensures the integrity of ASCII text during the steganographic process.
- **Flexible Character Sets**: Accommodates various character sets for versatile encoding and decoding.
- **Error Handling**: Comprehensive error handling for robust and reliable operations.

## Getting Started
To use Stego WPS in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
stego_wps = "1"
```

## Usage
Here's a quick overview of how to use Stego WPS in your Rust application:

### Encoding
```rust
use stego_wps::encode;

let text = "Your text here.";
let encoded = encode(text).expect("Failed to encode");
println!("Encoded text: {:?}", encoded);
```

### Decoding
```rust
use stego_wps::{decode, EncodingError};

let encoded_text = vec![...]; // Your encoded text
let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
let decoded = decode(&encoded_text, character_set).expect("Failed to decode");
println!("Decoded message: {}", decoded);
```

### Comparing
```rust
use stego_wps::compare;

let secret_message = "SECRET";
let cover_text = "Cover text with hidden message.";
let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
let comparison = compare(secret_message, cover_text, character_set).expect("Failed to compare");
println!("Comparison result: {:?}", comparison);
```

## License
This project is licensed under the LGPL-3.0-or-later.

## Contribution
Contributions to Stego WPS are welcome! Feel free to fork the repository, make your changes, and submit a pull request.

---

Stego WPS is an ongoing project, and we appreciate feedback and contributions from the Rust community. For more information, issues, or feature requests, please visit the [project repository](https://github.com/JamesClarke7283/stego_wps).
