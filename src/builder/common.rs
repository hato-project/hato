use std::fs;

pub fn create_dir(dir_path: &String) -> std::io::Result<()> {
    fs::create_dir_all(dir_path)?;
    Ok(())
}
