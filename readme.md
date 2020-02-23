# include_walk

[![Crate](https://img.shields.io/crates/v/include_walk.svg)](https://crates.io/crates/include_walk)
[![Rust](https://github.com/vincent-herlemont/include_walk/workflows/Rust/badge.svg)](https://github.com/vincent-herlemont/include_walk/actions/)

Include content files directory recursively using `include_str!` or `include_bytes!`.
It generate an output rust file with a method that return an [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

# Installation

Add `include_walk` to the build-dependencies in `./Cargo.toml`.
```toml
[build-dependencies]
include_walk = "0.1.1"
```

Create a builder file `./build.rs`. Below, there is a lite example : that import recursively all file present in `./src/assets/` and
generate a file import `./src/assets.rs`. 
By defaults, files are imported as `&'static str` using `include_str!`.
```rust
// ./build.rs
fn main() {
    include_walk::from("./src/assets/").to("./src/assets.rs");
}
```

### Use Cases

Retrieve all content files. For more detail see your generated file :
`./src/assets.rs` in that example.
```rust
// ./src/main.rs
mod assets;

fn main() {
    let assets = assets::getAll();
    let content = assets.get("assets/relative/path/to/files...").unwrap();
    println!("{}", content);
}
```

# Methods 

| Methods | Required | Default | Description  |
| ------- |:--------:|:-------:| ------------|
| ::from(path) | YES   | - | Specified the directory path to import |
| .to(path) | YES   | - | The path of generated module. |
| .filter(&#124;entry&#124; -> bool) | NO   | deactivate | Filter function that take a callback who can provide an `entry` argument and return `bool` : `true` for include and `false` for exclude file. |
| .bytes() | NO | deactivated | include with `include_bytes!` |
| .str() | NO | activated | include with `include_str!` |



