use std::{fs, path};
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;
use walkdir::WalkDir;

fn scan_directories(source_dir: &str, target_dir: &str) -> io::Result<()> {
    scan_directory(source_dir)?;
    scan_directory( target_dir)?;

    Ok(())
}
fn scan_directory(directory: &str) -> io::Result<()> {
    // iterator to iterate through the directory
    let dir_entries = WalkDir::new(directory);

    for entry in dir_entries  {
        let entry = entry?;

        let entry_path = entry.path();

        if entry_path.is_dir() {
            fs::create_dir_all(entry_path)?;
        }
    }
    Ok(())
}

fn copy_files(source_dir: &str, target_dir: &str) -> io::Result<()> {
    for entry in WalkDir::new(source_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => return Err(Error::new(ErrorKind::Other, format!("{}", err))),
        };
        let source_path = entry.path();
        let relative_path = match source_path.strip_prefix(source_dir) {
            Ok(rel_path) => rel_path,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Failed to calculate relative path".to_string(),
                ))
            }
        };
        let mut target_path = PathBuf::from(target_dir);
        target_path.push(relative_path);

        if source_path.is_file() {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(source_path, &target_path)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let source_dir = "/home/lukase/Downloads";
    let target_dir = "/run/media/lukase/USB-STICK/Backup";

    scan_directories(source_dir, target_dir);

    copy_files(source_dir, target_dir)?;
    println!("Backup completed successfully.");

    Ok(())
}
