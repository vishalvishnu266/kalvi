pub fn money(v: &f64) -> askama::Result<String> {
    let n = *v;
    let sign = if n < 0.0 { "-" } else { "" };
    let n = n.abs();
    let whole = n.trunc() as i64;
    let cents = ((n - whole as f64) * 100.0).round() as i64;

    let s = whole.to_string();
    let bytes = s.as_bytes();
    let mut with_sep = String::new();
    for (i, ch) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 {
            with_sep.push(',');
        }
        with_sep.push(*ch as char);
    }
    Ok(format!("{sign}{with_sep}.{cents:02}"))
}
