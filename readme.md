# include_walk

[![Rust](https://github.com/vincent-herlemont/include_walk/workflows/Rust/badge.svg)](https://github.com/vincent-herlemont/include_walk/actions/)

Include content files directory recursively using `include_str!` or `include_bytes!`.
It generate an output rust file with a method that return an [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

# Installation

The following example, import recursively all file present in `./src/assets/` and
generate a file import.
```rust
// build.rs
fn main() {
    include_walk::from("./src/assets/").to("./src/assets.rs");
}
```

You can customise many things, here the list of methods.

# Customise & Methods & Options 

| Methods | Required | Default | Description  |
| ------- |:--------:|:-------:| ------------|
| ::from(path) | YES   | - | Specified the directory path to import |
| .to(path) | YES   | - | The path of generated module. |
| .filter(&#124;entry&#124; -> bool) | NO   | deactivate | Filter function that take a callback who can provide an `entry` argument and return `bool` : `true` for include and `false` for exclude file. |
| .bytes() | NO | deactivate | include with `include_bytes!` |
| .str() | NO | activate | include with `include_str!` |