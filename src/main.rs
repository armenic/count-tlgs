use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

const DIRS: [&str; 3] = ["zzz/a", "zzz/d", "zzz/g"];
const FILES: [&str; 4] = ["t_bla.out", "l_01_bla.out", "l_02_bla.out", "g_bla.pdf"];

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

fn create_one_file(entry: &DirEntry, path: &str) -> io::Result<()> {
    if entry.file_type().is_dir() {
        let path: PathBuf = [entry.path().to_str().unwrap(), path].iter().collect();
        OpenOptions::new().create(true).write(true).open(path)?;
    }
    Ok(())
}

fn create_many_files(dir: &str) -> io::Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        for file in FILES {
            create_one_file(&entry, file)?;
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

    let mut outer_map = HashMap::new();

    let b_dirs: Vec<PathBuf> = WalkDir::new("zzz")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .filter(|e| e.file_name() == "b")
        .map(|e| e.path().to_owned())
        .collect();

    for bd in b_dirs {
        let mut groups = HashMap::new();

        for fe in WalkDir::new(&bd)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let f_name = String::from(fe.file_name().to_string_lossy());
            println!("{f_name}");
            let group = learn_io::group_tlg(&f_name);
            let counter = groups.entry(group).or_insert(0);
            *counter += 1;
        }

        outer_map.entry(bd.to_owned()).or_insert(groups.to_owned());
    }

    println!("{:?}", outer_map);

    Ok(())
}
