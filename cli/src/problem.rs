use serde::Serialize;
use std::path::Path;

/// One lint finding, reported as `file:line: message`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Problem {
    pub file: String,
    pub line: usize,
    pub check: &'static str,
    pub message: String,
}

impl Problem {
    pub fn new(
        root: &Path,
        file: &Path,
        line: usize,
        check: &'static str,
        message: String,
    ) -> Self {
        let relative = file.strip_prefix(root).unwrap_or(file);
        Problem {
            file: relative.to_string_lossy().replace('\\', "/"),
            line,
            check,
            message,
        }
    }
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: [{}] {}",
            self.file, self.line, self.check, self.message
        )
    }
}
