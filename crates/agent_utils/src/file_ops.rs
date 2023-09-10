use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};


pub fn write_to_file(filename: &str, text: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn read_txt_files(directory: &PathBuf) -> String {
    let mut all_text = String::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        all_text += &contents;
                        all_text += "\n";
                    }
                }
            }
        }
    }

    all_text
}
