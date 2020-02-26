use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn write(path: PathBuf, stmacro: String, entries: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut all_the_files = File::create(&path)?;

    writeln!(&mut all_the_files, r#"use std::collections::HashMap;"#,)?;
    writeln!(&mut all_the_files, r#""#,)?;
    writeln!(&mut all_the_files, r#"#[allow(dead_code)]"#,)?;
    writeln!(
        &mut all_the_files,
        r#"pub fn getAll() -> HashMap<&'static str, &'static str> {{"#,
    )?;
    writeln!(&mut all_the_files, r#"    let mut out = HashMap::new();"#,)?;

    for f in entries {
        writeln!(
            &mut all_the_files,
            r#"    out.insert("{path}", {stmacro}!("{path}"));"#,
            stmacro = stmacro,
            path = f.display()
        )?;
    }

    writeln!(&mut all_the_files, r#"    out"#,)?;
    writeln!(&mut all_the_files, r#"}}"#,)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple_write() {}
}
