use glob::{glob_with, MatchOptions};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

const DIRS: [&str; 3] = ["zzz/a", "zzz/d", "zzz/g"];
const FILES: [&str; 7] = [
    "t_bla.out",
    "t_bla.pdf",
    "t_bla.txt",
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
        let path: PathBuf = [dir, "data_analysis/primary_cSr/prod/output", "c"]
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
        // Level 1 files
        let mut de = fs::read_dir("zzz")?
            .map(|res| res.map(|e| e.path().into_os_string()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        de.sort();

        let mut expected: Vec<PathBuf> = ["a", "d", "g"]
            .iter()
            // PathBuf will properly use dividers based on OS
            .map(|d| PathBuf::from(String::from("zzz/") + d))
            .collect();
        expected.sort();

        assert_eq!(expected, de);

        // Level 2 files
        let mut de = fs::read_dir("zzz/g")?
            .map(|res| res.map(|e| e.path().into_os_string()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        de.sort();

        let mut expected: Vec<PathBuf> = [
            "data_analysis",
            "t_bla.out",
            "t_bla.pdf",
            "t_bla.txt",
            "eer_bla.out",
            "l_01_bla.out",
            "l_02_bla.out",
            "g_bla.pdf",
        ]
        .iter()
        // PathBuf will properly use dividers based on OS
        .map(|d| PathBuf::from(String::from("zzz/g/") + d))
        .collect();
        expected.sort();

        assert_eq!(expected, de);

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
mod tlg_tests {
    use super::*;

    #[test]
    fn test_tlg() {
        assert_eq!(group_tlg("l_abc.out"), "listing");
        assert_eq!(group_tlg("t_abc.out"), "table");
        assert_eq!(group_tlg("g_abc.out"), "graph");
        assert_eq!(group_tlg("ll_abc.out"), "other");
    }
}

pub fn prod_dirs(dir_name: &str, output_dir: &str) -> Vec<PathBuf> {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut f: Vec<PathBuf> = Vec::new();
    for entry in glob_with(&(String::from(dir_name) + output_dir), options).unwrap() {
        f.push(entry.unwrap());
    }

    f
}

#[cfg(test)]
mod prod_dirs_tests {
    use super::*;

    #[test]
    fn test_prod_dirs() {
        let mut b_dirs = prod_dirs("zzz", "/**/data_analysis/*csr*/prod/output/");
        b_dirs.sort();

        let expected: Vec<PathBuf> = [
            "zzz/a/data_analysis/primary_cSr/prod/output",
            "zzz/d/data_analysis/primary_cSr/prod/output",
            "zzz/g/data_analysis/primary_cSr/prod/output",
        ]
        .iter()
        // PathBuf will properly use dividers based on OS
        .map(|d| PathBuf::from(d))
        .collect();

        assert_eq!(b_dirs, expected)
    }
}

pub fn run(b_dirs: Vec<PathBuf>) -> io::Result<()> {
    let mut dir_names = HashMap::new();

    for bd in b_dirs {
        let mut counts = HashMap::new();

        // Need unique file names in case the same file exists in different
        // formats
        let mut file_names_stack: Vec<String> = Vec::new();

        for fe in WalkDir::new(&bd)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| ["out", "pdf"].contains(&e.path().extension().unwrap().to_str().unwrap()))
        {
            let f_name = String::from(fe.file_name().to_string_lossy());
            let f_base = fe.path().file_stem().unwrap().to_str().unwrap().to_string();
            if file_names_stack.contains(&f_base) {
                continue;
            }
            let group = group_tlg(&f_name);
            file_names_stack.push(f_base);
            let counter = counts.entry(group).or_insert(0);
            *counter += 1;
        }

        dir_names.insert(
            bd.parent().unwrap().parent().unwrap().to_owned(),
            counts.to_owned(),
        );
    }

    println!("{:#?}", dir_names);

    Ok(())
}

#[cfg(test)]
mod run_tests {
    use super::*;

    #[test]
    fn test_run() -> io::Result<()> {
        setup_test_files()?;
        let b_dirs = prod_dirs("zzz", "/**/data_analysis/*csr*/prod/output/");
        run(b_dirs)?;
        Ok(())
    }
}
