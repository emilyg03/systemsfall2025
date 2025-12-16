use std::{sync::{Arc, atomic::AtomicBool}, time::Instant, path::PathBuf};
use parallel_file_processor::{
    analyzers::{Analyzer, WordCount, LineCount, CharFrequency, FileSize},
    discover, processor, thread_pool::ThreadPool
};

fn main() {
    let files = discover::discover_files(&[PathBuf::from("books")]);
    let pool = ThreadPool::new(8);

    let analyzers: Arc<Vec<Box<dyn Analyzer>>> = Arc::new(vec![
        Box::new(WordCount),
        Box::new(LineCount),
        Box::new(CharFrequency),
        Box::new(FileSize),
    ]);

    let cancel = Arc::new(AtomicBool::new(false));
    let start = Instant::now();
    let (rx, progress) = processor::process_files_in_pool(&pool, files, analyzers, cancel);

    while let Ok(ev) = rx.recv() {
        if let processor::ProgressEvent::Finished { .. } = ev {
            let p = progress.lock().unwrap();
            if p.finished >= p.total { break; }
        }
    }

    pool.shutdown();
    println!("Total elapsed: {} ms", start.elapsed().as_millis());
}
