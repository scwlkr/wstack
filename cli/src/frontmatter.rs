//! Minimal `SKILL.md` frontmatter reader: flat `key: value` pairs between `---` fences.

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Frontmatter {
    pub name: Option<(usize, String)>,
    pub description: Option<(usize, String)>,
    /// Line of the opening fence, or `None` when the file has no frontmatter.
    pub start: Option<usize>,
    pub closed: bool,
}

fn unquote(value: &str) -> String {
    let value = value.trim();
    for quote in ['"', '\''] {
        if value.len() >= 2 && value.starts_with(quote) && value.ends_with(quote) {
            return value[1..value.len() - 1].replace("\\\"", "\"");
        }
    }
    value.to_string()
}

pub fn parse(text: &str) -> Frontmatter {
    let mut found = Frontmatter::default();
    let mut lines = text.lines().enumerate();
    match lines.next() {
        Some((_, first)) if first.trim_end() == "---" => found.start = Some(1),
        _ => return found,
    }
    for (index, line) in lines {
        if line.trim_end() == "---" {
            found.closed = true;
            break;
        }
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let entry = Some((index + 1, unquote(value)));
        match key.trim() {
            "name" => found.name = entry,
            "description" => found.description = entry,
            _ => {}
        }
    }
    found
}

/// Lowercase letters, digits and single hyphens; 1 to 64 characters.
pub fn valid_name(name: &str) -> bool {
    let well_formed = !name.is_empty()
        && name.len() <= 64
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--");
    well_formed
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}
