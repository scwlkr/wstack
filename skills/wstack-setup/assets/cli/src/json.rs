//! Serialize the small discovery schema without adding bootstrap dependencies.
pub fn string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            c if c < ' ' => output.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => output.push(c),
        }
    }
    output.push('"');
    output
}

pub fn strings(values: impl IntoIterator<Item = String>) -> String {
    format!(
        "[{}]",
        values
            .into_iter()
            .map(|s| string(&s))
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub fn optional(value: Option<String>) -> String {
    value.map(|v| string(&v)).unwrap_or_else(|| "null".into())
}
