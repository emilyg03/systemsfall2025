use std::{fs, path::Path};
use crate::types::{FileStats, ProcessingError};

pub trait Analyzer: Send + Sync {
    fn name(&self) -> &'static str;
    fn analyze(&self, path: &Path, content: &str, stats: &mut FileStats, errors: &mut Vec<ProcessingError>);
}

pub struct WordCount;
impl Analyzer for WordCount {
    fn name(&self) -> &'static str { "WordCount" }
    fn analyze(&self, _path: &Path, content: &str, stats: &mut FileStats, _errors: &mut Vec<ProcessingError>) {
        stats.word_count = content.split_whitespace().count();
    }
}

pub struct LineCount;
impl Analyzer for LineCount {
    fn name(&self) -> &'static str { "LineCount" }
    fn analyze(&self, _path: &Path, content: &str, stats: &mut FileStats, _errors: &mut Vec<ProcessingError>) {
        stats.line_count = content.lines().count();
    }
}

pub struct CharFrequency;
impl Analyzer for CharFrequency {
    fn name(&self) -> &'static str { "CharFrequency" }
    fn analyze(&self, _path: &Path, content: &str, stats: &mut FileStats, _errors: &mut Vec<ProcessingError>) {
        for ch in content.chars() {
            *stats.char_frequencies.entry(ch).or_insert(0) += 1;
        }
    }
}

pub struct FileSize;
impl Analyzer for FileSize {
    fn name(&self) -> &'static str { "FileSize" }
    fn analyze(&self, path: &Path, _content: &str, stats: &mut FileStats, errors: &mut Vec<ProcessingError>) {
        match fs::metadata(path) {
            Ok(m) => stats.size_bytes = m.len(),
            Err(e) => errors.push(ProcessingError {
                path: path.to_path_buf(),
                context: format!("metadata error: {e}"),
            }),
        }
    }
}
