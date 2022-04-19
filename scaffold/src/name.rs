/// Validate a portable Cargo package name without accepting paths.
pub fn validate_name(name: &str) -> Result<(), String> {
    let mut bytes = name.bytes();
    if !bytes.next().is_some_and(|c| c.is_ascii_alphabetic())
        || !bytes.all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    {
        return Err(format!("invalid project name {name:?}: use an ASCII letter followed by letters, digits, hyphens or underscores"));
    }
    Ok(())
}
