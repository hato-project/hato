use std::fs;
use std::io;

pub fn create_dir(dir_path: &String) -> io::Result<()> {
    fs::create_dir_all(dir_path)?;
    Ok(())
}
