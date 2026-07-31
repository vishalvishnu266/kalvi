//! Regenerate `lit-components/dsl-attendance.html`.
//!
//! ```sh
//! cargo run -p lit-ui --example attendance
//! # open http://localhost:3000/lit-components/dsl-attendance.html
//! ```

use lit_ui::core::Component;
use lit_ui::pages::attendance;
use std::{fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let data = attendance::mock_class();
    let html = attendance::build("Grade 5-B", "2026-08-14", &data).render();

    let out = output_path("dsl-attendance.html");
    fs::create_dir_all(out.parent().unwrap())?;
    fs::write(&out, &html)?;
    eprintln!("wrote {} bytes → {}", html.len(), out.display());
    eprintln!("open  → http://localhost:3000/lit-components/dsl-attendance.html");
    Ok(())
}

fn output_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .join("lit-components").join(name)
}
