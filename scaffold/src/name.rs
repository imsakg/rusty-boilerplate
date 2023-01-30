/// Validate a portable Cargo package name without accepting paths.
pub fn validate_name(name: &str) -> Result<(), String> {
    let mut bytes = name.bytes();
    if !bytes.next().is_some_and(|c| c.is_ascii_alphabetic())
        || !bytes.all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    {
        return Err(format!("invalid project name {name:?}: use an ASCII letter followed by letters, digits, hyphens or underscores"));
    }
    let future_reserved = [
        "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof",
        "unsized", "virtual", "yield", "union",
    ];
    if future_reserved.contains(&name) {
        return Err(format!("reserved project name {name:?}"));
    }
    let reserved = [
        "self", "Self", "super", "crate", "mod", "type", "fn", "use", "pub", "let", "const",
        "static", "enum", "struct", "trait", "impl", "where", "match", "move", "async", "await",
        "dyn", "ref", "return", "loop", "while", "for", "in", "if", "else", "break", "continue",
        "as", "extern", "unsafe", "true", "false",
    ];
    let lower = name.to_ascii_lowercase();
    let device = ["con", "prn", "aux", "nul"].contains(&lower.as_str())
        || ((lower.starts_with("com") || lower.starts_with("lpt"))
            && lower.len() == 4
            && matches!(lower.as_bytes()[3], b'1'..=b'9'));
    if reserved.contains(&name) || device || name.contains("--") {
        return Err(format!("reserved or ambiguous project name {name:?}"));
    }
    Ok(())
}
