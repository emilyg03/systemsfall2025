mod analyzers;
mod discover;
mod processor;
mod thread_pool;
mod types;

use std::{
    env,
    path::PathBuf,
    sync::{atomic::{AtomicBool, Ordering}, Arc},
    thread,
};

use analyzers::{CharFrequency, FileSize, LineCount, WordCount, Analyzer};
use processor::{ProgressEvent};
use thread_pool::ThreadPool;

fn main() {
    let mut args = env::args().skip(1);
    let threads: usize = args.next().unwrap_or_else(|| "4".into()).parse().unwrap_or(4);
    let dirs: Vec<PathBuf> = args.map(PathBuf::from).collect();

    let dirs = if dirs.is_empty() { vec![PathBuf::from("books")] } else { dirs };

    let files = discover::discover_files(&dirs);
    println!("Discovered {} files", files.len());

    if files.len() < 100 {
        eprintln!("WARNING: need at least 100 Gutenberg books. Currently: {}", files.len());
    }

    let analyzers: Arc<Vec<Box<dyn Analyzer>>> = Arc::new(vec![
        Box::new(WordCount),
        Box::new(LineCount),
        Box::new(CharFrequency),
        Box::new(FileSize),
    ]);

    let cancel = Arc::new(AtomicBool::new(false));

    {
        let cancel = Arc::clone(&cancel);
        thread::spawn(move || {
            use std::io::{self, Read};
            let _ = io::stdin().read(&mut [0u8]).ok();
            cancel.store(true, Ordering::Relaxed);
            eprintln!("Cancellation requested.");
        });
    }

    let pool = ThreadPool::new(threads);

    let (rx, progress) = processor::process_files_in_pool(&pool, files, analyzers, cancel);

    let mut results = Vec::new();

    while let Ok(ev) = rx.recv() {
        match ev {
            ProgressEvent::Started { path } => {
                let p = progress.lock().unwrap();
                println!("[{}/{} started] {}", p.started, p.total, path.display());
            }
            ProgressEvent::SkippedCancelled { path } => {
                println!("[cancelled] {}", path.display());
            }
            ProgressEvent::Finished { analysis } => {
                let p = progress.lock().unwrap();
                println!(
                    "[{}/{} finished] {} ({} ms, errors={})",
                    p.finished,
                    p.total,
                    analysis.filename,
                    analysis.processing_time.as_millis(),
                    analysis.errors.len()
                );
                results.push(analysis);

                if p.finished >= p.total {
                    break;
                }
            }
        }
    }

    
    pool.shutdown();

    // Summary
    let p = progress.lock().unwrap();
    println!("--- SUMMARY ---");
    println!("Total: {}", p.total);
    println!("Finished: {}", p.finished);
    println!("Errors: {}", p.errors);
    if p.finished > 0 {
        println!("Avg time/file: {} ms", (p.total_time.as_millis() / p.finished as u128));
    }


}
