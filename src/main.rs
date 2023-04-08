use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::PathBuf;
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

#[allow(dead_code)]
fn group_tlg(file_name: &str) -> String {
    let table = file_name.starts_with("t_");
    let listing = file_name.starts_with("l_");
    let graph = file_name.starts_with("g_");

    let temp = if table {
        "t_"
    } else if listing {
        "l_"
    } else if graph {
        "g_"
    } else {
        "o_"
    };

    String::from(temp)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(group_tlg("l_abc.out"), "l_");
        assert_eq!(group_tlg("t_abc.out"), "t_");
        assert_eq!(group_tlg("g_abc.out"), "g_");
        assert_eq!(group_tlg("ll_abc.out"), "o_");
    }
}

fn main() -> io::Result<()> {
    clean_many_dirs().unwrap_or_default();
    create_many_dirs()?;
    traverse_dirs()?;

    let mut filenames = HashMap::new();

    for entry in WalkDir::new("zzz")
        .into_iter()
        .filter_map(Result::ok)
        // TODO
        // We might want to only search within directory b
        .filter(|e| e.file_type().is_file())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let group = group_tlg(&f_name);
        let counter = filenames.entry(group.clone()).or_insert(0);
        *counter += 1;
    }

    println!("{:?}", filenames);

    Ok(())
}
