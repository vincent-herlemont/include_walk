use include_walk::walk;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::Command;
use tempdir::TempDir;

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new("include_walk_test")?;

    //Create fake directories
    {
        let pds = vec![
            dir.path().join("d1/"),
            dir.path().join("d1/d2/"),
            dir.path().join("src/"),
        ];
        for pd in pds {
            create_dir_all(pd)?;
        }
    }
    //Create fake files
    {
        let main_content = format!(
            r#"mod assets;
fn main() {{
     let a = assets::assets();
     println!("c:{{}}", a.capacity());
     println!("assert f1:{{}}", a.contains_key("{p}/d1/d2/f4.txt"));
     println!("assert f1:content:{{}}", a.get("{p}/d1/d2/f4.txt").unwrap());
}}
"#,
            p = &dir.path().to_string_lossy()
        );
        let main_content = main_content.as_str();
        let pfs = vec![
            (dir.path().join("f1.txt"), ""),
            (dir.path().join("f2.txt"), ""),
            (dir.path().join("d1/f3.txt"), ""),
            (dir.path().join("d1/d2/f4.txt"), "content f4"),
            (
                dir.path().join("Cargo.toml"),
                r#"[package]
name = "include_walk_test"
version = "0.0.1"
authors = ["Vincent Herlemont <vincentherl@leszeros.com>"]
edition = "2018"

[dependencies]
"#,
            ),
            (dir.path().join("src/main.rs"), main_content),
        ];

        for (pf, content) in pfs {
            let mut f = File::create(&pf)?;
            f.write_all(content.as_bytes())?
        }
    }

    let assetfile_paths = &dir.path().join("src/assets.rs");
    walk(&dir.path()).str().to(assetfile_paths)?;

    let cmd = Command::new("cargo")
        .arg("run")
        .current_dir(&dir.path())
        .output()
        .unwrap();
    assert!(cmd.status.success());
    let out = String::from_utf8(cmd.stdout)?;
    assert_eq!(
        out.as_str(),
        "c:7\nassert f1:true\nassert f1:content:content f4\n"
    );

    Ok(())
}
