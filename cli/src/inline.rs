//! Inline blocks: a shared file embedded between marker lines inside a skill's `SKILL.md`.

/// Marker line that opens the block for the shared file `label` (for example `shared/principles.md`).
pub fn begin(label: &str) -> String {
    format!("<!-- wstack:begin {label} -->")
}

/// Marker line that closes the block for `label`.
pub fn end(label: &str) -> String {
    format!("<!-- wstack:end {label} -->")
}

/// 0-based line indexes of the begin and end markers; `Err` says what is wrong.
pub fn locate(text: &str, label: &str) -> Result<(usize, usize), String> {
    let (open, close) = (begin(label), end(label));
    let find = |marker: &str| text.lines().position(|line| line.trim() == marker);
    match (find(&open), find(&close)) {
        (Some(a), Some(b)) if a < b => Ok((a, b)),
        (Some(_), Some(_)) => Err(format!("`{close}` comes before `{open}`")),
        _ => Err(format!("missing markers `{open}` and `{close}`")),
    }
}

/// `text` with the lines between the markers replaced by `block`.
pub fn render(text: &str, label: &str, block: &str) -> Result<String, String> {
    let (open, close) = locate(text, label)?;
    let lines: Vec<&str> = text.lines().collect();
    let mut out: Vec<&str> = lines[..=open].to_vec();
    out.extend(block.lines());
    out.extend(&lines[close..]);
    Ok(out.join("\n") + "\n")
}
