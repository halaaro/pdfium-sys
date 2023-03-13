# PDFium low-level bindings for Rust

> WARNING: This crate is very much a work in progress in its current state.

## Features

**`dynamic_link`** - Link to dynamic library instead of default static lib.

If not installed in a system location, ensure enviroment variable `PDFIUM_LIB_PATH` is set to the location of the PDFium dynamic library. The library name should be `libpdfium.so` or `pdfium.dll` (on Windows). Feature is ignored when `pdfium_build` enabled.

**`pdfium_build`** (EXPERIMENTAL) - Build PDFium static library from sources.

By default dependent sources and tools are downloaded to a location where `cargo clean` will remove them. These environment variables can be used to override the default location to avoid this behavior:

| Enviroment variable    | Meaning                           |
| ---                    | ---                               |
| `PDFIUM_GCLIENT_CACHE` | Cache location used by `gclient`. |
| `PDFIUM_GCLIENT_BUILD` | Build location for dependencies.  |

**`bindgen`** - Generate bindings from sources.

Must have Clang installed to run `bindgen`. See [bindgen requirements](https://rust-lang.github.io/rust-bindgen/requirements.html) for details.

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
