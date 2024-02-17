use std::{
    fmt::{self, Display},
    fs,
    path::PathBuf,
};

use colored::Colorize;
use glob::glob;

#[derive(Debug)]
pub struct RGrep {
    pub paths: Vec<PathBuf>,
    pub search: String,
}

pub struct Output {
    pub file: String,
    pub lines: Vec<Line>,
}

pub struct Line {
    pub idx: usize,
    pub line: String,
}

impl Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.file.red())?;

        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

impl Line {
    pub fn new(idx: usize, line: impl Into<String>) -> Self {
        Self {
            idx,
            line: line.into(),
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", format!("{}", self.idx).green(), self.line)
    }
}

impl RGrep {
    pub fn new(search: String, file_glob: String) -> Self {
        let entries = glob(&file_glob).expect("Failed to read glob pattern");

        let pwd = std::env::current_dir().unwrap();

        let paths: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|path| pwd.join(path))
            .collect();

        Self { paths, search }
    }

    pub fn match_files(&mut self) -> Vec<Output> {
        let mut outputs = vec![];

        let search = self.search.clone();
        let search = search.as_str();
        let pattern = regex::Regex::new(search).unwrap();

        for (name, content) in self {
            let mut output = Output {
                file: name,
                lines: vec![],
            };

            for (idx, line) in content.lines().enumerate() {
                if pattern.is_match(line) {
                    output.lines.push(Line::new(
                        idx + 1,
                        line.replace(search, &search.green().to_string()),
                    ));
                }
            }

            if !output.lines.is_empty() {
                outputs.push(output);
            }
        }

        outputs
    }

    pub fn print(&mut self) {
        let outputs = self.match_files();

        for output in outputs {
            println!("{}", output);
        }
    }
}

impl Iterator for RGrep {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let path = self.paths.pop();

        match path {
            Some(p) => Some((p.display().to_string(), fs::read_to_string(p).unwrap())),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Args;

    #[test]
    fn get_paths_should_work() {
        let args = Args {
            file_glob: "fixtures/*.txt".to_string(),
            search: "".to_string(),
        };

        let grep = RGrep::new(args.search, args.file_glob);

        let paths = grep.paths;

        assert!(paths.len() > 0);
    }

    #[test]
    fn match_files_should_work() {
        let args = Args {
            file_glob: "fixtures/*.txt".to_string(),
            search: "code".to_string(),
        };

        let mut grep = RGrep::new(args.search, args.file_glob);

        let output = grep.match_files();

        for o in output {
            println!("{}", o);
        }
    }
}
