# include_walk

[![Rust](https://github.com/vincent-herlemont/include_walk/workflows/Rust/badge.svg)](https://github.com/vincent-herlemont/include_walk/actions/)

Include content files directory recursively using `include_str!` or `include_bytes`.
It generate an output rust file with a method that return an [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

# Installation

Use `walk` method in your `build.rs` you can customise with :
 - filtered the files (`.filter(|e| -> ..your filter..)`),
 - change the name of get `method` (`.method("get_assets")`)
 - choice the cast : `.str()` or `.bytes()`
 - output path of generated file : `.to("./src/assets.rs")`
```rust
// ./build.rs
use include_walk::walk;

fn main() {
    // Example
    walk("./")
        .filter(|e| e.path().to_string_lossy().contains("assets"))
        .method("get_assets")
        .str()
        .to("./src/assets.rs")
        .unwrap();
}
```

output : 
```rust
// ./src/assets.rs
use std::collections::HashMap;

#[allow(dead_code)]
pub fn get_assets() -> HashMap<&'static str, &'static str> {
    let mut out = HashMap::new();
    out.insert("./src/_assets.rs", include_str!("./src/_assets.rs"));
    out.insert("./src/assets.rs", include_str!("./src/assets.rs"));
    out.insert("./src/assets/certificate.yaml", include_str!("./src/assets/certificate.yaml"));
    out
}
```