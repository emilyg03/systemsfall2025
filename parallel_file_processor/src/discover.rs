
use std::{fs, path::{Path, PathBuf}};


pub fn discover_files(dirs: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for d in dirs {
        walk(d, &mut out);
    }
    out
}

fn walk(path: &Path, out: &mut Vec<PathBuf>) {
    let rd = match fs::read_dir(path) {
        Ok(x) => x,
        Err(_) => return, 
    };

    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.is_file() {
            out.push(p);
        }
    }
}
