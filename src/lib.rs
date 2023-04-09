use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

const DIRS: [&str; 3] = ["zzz/a", "zzz/d", "zzz/g"];
const FILES: [&str; 5] = [
    "t_bla.out",
    "eer_bla.out",
    "l_01_bla.out",
    "l_02_bla.out",
    "g_bla.pdf",
];

fn clean_many_dirs() -> io::Result<()> {
    for dir in DIRS {
        fs::remove_dir_all(dir)?
    }

    Ok(())
}

fn create_many_dirs() -> io::Result<()> {
    for dir in DIRS {
        let path: PathBuf = [dir, "data_analysis/primary_csr/prod/output", "c"]
            .iter()
            .collect();
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

pub fn setup_test_files() -> io::Result<()> {
    clean_many_dirs().unwrap_or_default();
    create_many_dirs()?;
    traverse_dirs()?;
    Ok(())
}

#[cfg(test)]
mod setup_tests {
    use super::*;

    #[test]
    fn test_setup() -> io::Result<()> {
        setup_test_files()?;
        Ok(())
    }
}

fn group_tlg(file_name: &str) -> String {
    let first_two_chars: String = file_name.chars().take(2).collect();

    let temp = match first_two_chars.as_str() {
        "t_" => "table",
        "l_" => "listing",
        "g_" => "graph",
        _ => "other",
    };

    String::from(temp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tlg() {
        assert_eq!(group_tlg("l_abc.out"), "listing");
        assert_eq!(group_tlg("t_abc.out"), "table");
        assert_eq!(group_tlg("g_abc.out"), "graph");
        assert_eq!(group_tlg("ll_abc.out"), "other");
    }
}

pub fn run(dir_name: &str) -> io::Result<()> {
    let mut outer_map = HashMap::new();

    let b_dirs: Vec<PathBuf> = WalkDir::new(dir_name)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .filter(|e| {
            e.path().components().any(|c| {
                c.as_os_str()
                    .to_string_lossy()
                    .to_lowercase()
                    .contains("csr")
            })
        })
        .filter(|e| e.path().components().any(|c| c.as_os_str() == "prod"))
        .filter(|e| {
            e.path()
                .components()
                .any(|c| c.as_os_str() == "data_analysis")
        })
        .filter(|e| e.file_name() == "output")
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
            let group = group_tlg(&f_name);
            let counter = groups.entry(group).or_insert(0);
            *counter += 1;
        }

        outer_map.insert(bd.to_owned(), groups.to_owned());
    }

    println!("{:#?}", outer_map);

    Ok(())
}
