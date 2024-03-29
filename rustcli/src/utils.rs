use std::fs;
use std::fs::{File, Metadata};
use std::io::Read;
use std::path::Path;

pub fn echo_function(input: &str){
    if input.is_empty() {
        String::new();
    } else {
        println!("{}", input);
    }
}

pub fn ls_function(input: &str) -> Result<Vec<String>, std::io::Error>{
        let mut entries_list = Vec::new();
        let dir = Path::new(input);
        if dir.is_dir() {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(nom_fic) = entry.file_name().into_string() {
                            entries_list.push(nom_fic);
                        } else {
                            entries_list.push("Fichier corrompu".to_string());
                        }
                    }
                }
            }
            Ok(entries_list)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a directory"))
        }
    }

pub fn cat_function(file_names: Vec<String>) -> Result<Vec<String>, std::io::Error> {
    let mut files_contents = Vec::new();
    for file_name in &file_names {
        let mut file_content = String::new();
        let file_path = Path::new(file_name);
        let mut file = File::open(file_path)?;
        file.read_to_string(&mut file_content)?;
        files_contents.push(file_content);
    }
    Ok(files_contents)
}

