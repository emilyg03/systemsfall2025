use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    time::{Duration, Instant},
};

use crate::{
    analyzers::Analyzer,
    types::{FileAnalysis, FileStats, ProcessingError},
};

#[derive(Debug, Clone)]
pub enum ProgressEvent {
    Started { path: PathBuf },
    Finished { analysis: FileAnalysis },
    SkippedCancelled { path: PathBuf },
}

#[derive(Debug, Default)]
pub struct Progress {
    pub total: usize,
    pub started: usize,
    pub finished: usize,
    pub errors: usize,
    pub total_time: Duration,
}

pub fn process_files_in_pool(
    pool: &crate::thread_pool::ThreadPool,
    files: Vec<PathBuf>,
    analyzers: Arc<Vec<Box<dyn Analyzer>>>,
    cancel: Arc<AtomicBool>,
) -> (mpsc::Receiver<ProgressEvent>, Arc<Mutex<Progress>>) {
    let (tx, rx) = mpsc::channel::<ProgressEvent>();
    let progress = Arc::new(Mutex::new(Progress { total: files.len(), ..Default::default() }));

    for path in files {
        let tx = tx.clone();
        let analyzers = Arc::clone(&analyzers);
        let cancel = Arc::clone(&cancel);
        let progress = Arc::clone(&progress);

        pool.execute(move || {
            if cancel.load(Ordering::Relaxed) {
                let _ = tx.send(ProgressEvent::SkippedCancelled { path });
                return;
            }

            {
                let mut p = progress.lock().unwrap();
                p.started += 1;
            }
            let _ = tx.send(ProgressEvent::Started { path: path.clone() });

            let start = Instant::now();
            let mut errors: Vec<ProcessingError> = Vec::new();
            let mut stats = FileStats::default();

            // ReadFile
            let content = match std::fs::read(&path) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(e) => {
                        errors.push(ProcessingError {
                            path: path.clone(),
                            context: format!("utf8 decode error: {e} (using lossy)"),
                        });
                        // lossy fallback (bonus-ish)
                        String::from_utf8_lossy(e.as_bytes()).to_string()
                    }
                },
                Err(e) => {
                    errors.push(ProcessingError {
                        path: path.clone(),
                        context: format!("read error: {e}"),
                    });
                    String::new()
                }
            };

            // Run 
            for a in analyzers.iter() {
                if cancel.load(Ordering::Relaxed) {
                    let _ = tx.send(ProgressEvent::SkippedCancelled { path });
                    return;
                }
                a.analyze(&path, &content, &mut stats, &mut errors);
            }

            let elapsed = start.elapsed();
            let filename = path.to_string_lossy().to_string();
            let analysis = FileAnalysis { filename, stats, errors: errors.clone(), processing_time: elapsed };

            {
                let mut p = progress.lock().unwrap();
                p.finished += 1;
                p.total_time += elapsed;
                if !errors.is_empty() {
                    p.errors += errors.len();
                }
            }

            let _ = tx.send(ProgressEvent::Finished { analysis });
        });
    }

    (rx, progress)
}
