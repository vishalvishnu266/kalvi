//! Regenerate `lit-components/dsl-dashboard.html`.
//!
//! ```sh
//! cargo run -p lit-ui --example dashboard
//! # open http://localhost:3000/lit-components/dsl-dashboard.html
//! ```

use lit_ui::core::Component;
use lit_ui::pages::dashboard;
use std::{fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let html = dashboard::build().render();

    let out = output_path("dsl-dashboard.html");
    fs::create_dir_all(out.parent().unwrap())?;
    fs::write(&out, &html)?;
    eprintln!("wrote {} bytes → {}", html.len(), out.display());
    eprintln!("open  → http://localhost:3000/lit-components/dsl-dashboard.html");
    Ok(())
}

fn output_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .join("lit-components").join(name)
}
