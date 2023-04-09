use std::io;

fn main() -> io::Result<()> {
    learn_io::setup_test_files()?;
    learn_io::run()?;
    Ok(())
}
