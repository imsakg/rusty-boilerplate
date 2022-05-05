use std::collections::BTreeMap;

/// Substitute explicit `{{key}}` variables without interpreting Rust braces.
pub fn render(source: &str, variables: &BTreeMap<String, String>) -> Result<String, String> {
    let mut output = source.to_owned();
    for (key, value) in variables {
        output = output.replace(&format!("{{{{{key}}}}}"), value);
    }
    Ok(output)
}
