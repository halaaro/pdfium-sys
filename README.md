# PDFium low-level bindings for Rust

Relies on `pdfium.dll` to be available. Tested against binaries from https://github.com/bblanchon/pdfium-binaries.

> Only tested on Windows. Pull requests welcome!

## Building from Source

Must have Clang installed to run `bindgen`. See [bindgen requirements](https://rust-lang.github.io/rust-bindgen/requirements.html) for details.

Ensure the pdfium header `fpdfview.h` is available. In Windows it is enough to add its path to the `INCLUDE` environment variable before calling `cargo build`.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

---

This project is not affilated with either Google or Foxit.
