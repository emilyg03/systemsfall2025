use std::{collections::HashMap, path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
pub struct FileAnalysis {
    pub filename: String,
    pub stats: FileStats,
    pub errors: Vec<ProcessingError>,
    pub processing_time: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct FileStats {
    pub word_count: usize,
    pub line_count: usize,
    pub char_frequencies: HashMap<char, usize>,
    pub size_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessingError {
    pub path: PathBuf,
    pub context: String,
}