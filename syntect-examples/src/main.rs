use anyhow::{anyhow, Result};
use std::{
    fs::{self},
    path::Path,
};

use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

fn main() {
    let files: Vec<_> = walkdir::WalkDir::new("fixtures")
        .into_iter()
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .filter(|e| e.path().is_file())
        .collect();

    for entry in files {
        let path = entry.path();

        highlight_file(path).unwrap();
    }
}

fn highlight_file(path: &Path) -> Result<()> {
    let ext = path.extension().unwrap().to_str().unwrap();

    let contents = fs::read_to_string(path).unwrap();

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ss.find_syntax_by_extension(ext).unwrap_or_else(|| {
        let line = contents.lines().next().unwrap_or("");

        ss.find_syntax_by_first_line(&line)
            .ok_or_else(|| anyhow!("No syntax found"))
            .unwrap_or(ss.find_syntax_plain_text())
    });

    let mut h = HighlightLines::new(syntax, ts.themes.get("base16-ocean.dark").unwrap());
    let contents = fs::read_to_string(path)?;

    for line in LinesWithEndings::from(contents.as_str()) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);

        print!("{}", escaped);
    }

    Ok(())
}
