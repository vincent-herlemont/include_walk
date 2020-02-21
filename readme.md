# include_walk

WIP

```rust
// ./build.rs
use include_walk::walk;

fn main() {
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
use utils::asset::Asset;

#[allow(dead_code)]
pub fn get_assets() -> Vec<Asset> {
    vec![
        Asset::new(
            "./src/assets.rs",
            include_str!("./src/assets.rs"),
        ),
        Asset::new(
            "./src/assets/certificate.yaml",
            include_str!("./src/assets/certificate.yaml"),
        ),
        Asset::new(
            "./assets.rs",
            include_str!("./assets.rs"),
        ),
    ]
}

```