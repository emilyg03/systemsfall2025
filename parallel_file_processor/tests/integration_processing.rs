use std::{fs, path::PathBuf, sync::{Arc, atomic::{AtomicBool, Ordering}}};

use parallel_file_processor::{
    analyzers::{Analyzer, WordCount, LineCount, CharFrequency, FileSize},
    processor,
    thread_pool::ThreadPool,
};

#[test]
fn processes_files_end_to_end() {
    let dir = std::env::temp_dir().join("p_fp_test");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let f1 = dir.join("a.txt");
    fs::write(&f1, "hello world\nsecond line").unwrap();

    let pool = ThreadPool::new(2);
    let analyzers: Arc<Vec<Box<dyn Analyzer>>> = Arc::new(vec![
        Box::new(WordCount),
        Box::new(LineCount),
        Box::new(CharFrequency),
        Box::new(FileSize),
    ]);

    let cancel = Arc::new(AtomicBool::new(false));
    let (rx, _progress) = processor::process_files_in_pool(&pool, vec![f1.clone()], analyzers, cancel);

    let mut got = None;
    while let Ok(ev) = rx.recv() {
        if let processor::ProgressEvent::Finished { analysis } = ev {
            got = Some(analysis);
            break;
        }
    }

    pool.shutdown();

    let analysis = got.expect("no analysis received");
    assert!(analysis.stats.word_count >= 2);
    assert!(analysis.stats.line_count >= 2);
    assert!(analysis.stats.size_bytes > 0);
}
