use std::{fs, path::PathBuf};

use glob::glob;

#[derive(Debug)]
pub struct RGrep {
    pub paths: Vec<PathBuf>,
    pub search: String,
}

impl RGrep {
    pub fn new(search: String, file_glob: String) -> Self {
        let entries = glob(&file_glob).expect("Failed to read glob pattern");

        let pwd = std::env::current_dir().unwrap();

        let paths: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|path| pwd.join(path.to_string_lossy().to_string()))
            .collect();

        Self { paths, search }
    }

    pub fn match_files(&mut self) -> Vec<String> {
        let mut matched_files = vec![];

        let search = self.search.clone();
        let search = search.as_str();

        for file in self {
            for line in file.lines() {
                if line.contains(search) {
                    matched_files.push(line.to_string());
                }
            }
        }

        matched_files
    }
}

impl Iterator for RGrep {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let path = self.paths.pop();

        match path {
            Some(p) => fs::read_to_string(p).ok(),
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

        let matches = grep.match_files();

        println!("{:?}", matches);
    }
}
