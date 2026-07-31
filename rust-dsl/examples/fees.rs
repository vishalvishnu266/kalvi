//! Regenerate `lit-components/dsl-fees.html`.
//!
//! ```sh
//! cargo run -p lit-ui --example fees
//! # open http://localhost:3000/lit-components/dsl-fees.html
//! ```

use lit_ui::core::Component;
use lit_ui::pages::fees;
use std::{fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let data = fees::mock_invoices();
    let html = fees::build(&data).render();

    let out = output_path("dsl-fees.html");
    fs::create_dir_all(out.parent().unwrap())?;
    fs::write(&out, &html)?;
    eprintln!("wrote {} bytes → {}", html.len(), out.display());
    eprintln!("open  → http://localhost:3000/lit-components/dsl-fees.html");
    Ok(())
}

fn output_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .join("lit-components").join(name)
}
