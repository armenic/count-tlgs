use std::{env, io, process};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide the directory name!");
        process::exit(1)
    }

    let dir_name = &args[1];
    let default_output = &String::from("/**/data_analysis/*csr*/prod/output/");

    let output_dir = if args.len().clone() == 3 {
        &args[2]
    } else {
        &default_output
    };

    println!("Searching directory {} for TLGs", &dir_name);

    let b_dirs = count_tlgs::prod_dirs(&dir_name, &output_dir);
    count_tlgs::run(b_dirs)?;
    Ok(())
}
