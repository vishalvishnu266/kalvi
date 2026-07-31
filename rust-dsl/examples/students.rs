//! Regenerate `lit-components/dsl-students.html` from `lit_ui::pages::students`.
//!
//! ```sh
//! cargo run -p lit-ui --example students
//! # open http://localhost:3000/lit-components/dsl-students.html
//! ```

use lit_ui::core::Component;
use lit_ui::pages::students;
use std::{fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let data = students::mock_students();
    let html = students::build(&data).render();

    let out = output_path("dsl-students.html");
    fs::create_dir_all(out.parent().unwrap())?;
    fs::write(&out, &html)?;
    eprintln!("wrote {} bytes → {}", html.len(), out.display());
    eprintln!("open  → http://localhost:3000/lit-components/dsl-students.html");
    Ok(())
}

fn output_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .join("lit-components").join(name)
}
