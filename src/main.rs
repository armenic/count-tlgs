use std::{env, io, process};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide the directory name!");
        process::exit(1)
    }

    println!("Searching directory {} for TLGs", args[1]);

    learn_io::setup_test_files()?;
    learn_io::run(&args[1])?;
    Ok(())
}
