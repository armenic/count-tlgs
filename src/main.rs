use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

const DIRS: [&str; 3] = ["zzz/a", "zzz/d", "zzz/g"];
const FILES: [&str; 3] = ["t_bla.out", "l_bla.out", "g_bla.pdf"];

fn clean_many_dirs() -> io::Result<()> {
    for dir in DIRS {
        fs::remove_dir_all(dir)?
    }

    Ok(())
}

fn create_many_dirs() -> io::Result<()> {
    for dir in DIRS {
        let path: PathBuf = [dir, "b", "c"].iter().collect();
        fs::create_dir_all(path)?;
    }

    Ok(())
}

fn create_one_file(entry: &DirEntry, path: &Path) -> io::Result<()> {
    if entry.file_type().is_dir() {
        let path: PathBuf = [entry.path(), path].iter().collect();
        OpenOptions::new().create(true).write(true).open(path)?;
    }
    Ok(())
}

fn create_many_files(dir: &str) -> io::Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        for file in FILES {
            create_one_file(&entry, Path::new(file))?;
        }
    }
    Ok(())
}

fn traverse_dirs() -> io::Result<()> {
    for dir in DIRS {
        create_many_files(dir)?
    }
    Ok(())
}

fn main() -> io::Result<()> {
    clean_many_dirs().unwrap_or_default();
    create_many_dirs()?;
    traverse_dirs()?;

    for entry in WalkDir::new("a") {
        let entry = entry.unwrap();
        println!("{}", entry.path().display())
    }

    Ok(())
}
