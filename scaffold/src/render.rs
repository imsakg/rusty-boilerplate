use std::collections::BTreeMap;

/// Substitute explicit `{{key}}` variables in a single pass.
pub fn render(source: &str, variables: &BTreeMap<String, String>) -> Result<String, String> {
    let mut output = String::new();
    let mut rest = source;
    while let Some(start) = rest.find("{{") {
        output.push_str(&rest[..start]);
        rest = &rest[start + 2..];
        let end = rest.find("}}").ok_or("unclosed template variable")?;
        let key = rest[..end].trim();
        let value = variables
            .get(key)
            .ok_or_else(|| format!("unknown template variable {key:?}"))?;
        output.push_str(value);
        rest = &rest[end + 2..];
    }
    output.push_str(rest);
    Ok(output)
}
