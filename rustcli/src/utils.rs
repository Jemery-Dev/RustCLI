use std::fs;
use std::fs::{File, Metadata};
use std::io::Read;
use std::path::Path;

/// Param string
/// Return, println of string
/// Prend un string et l'affiche

pub fn echo_function(input: &str){
    if input.is_empty() {
        let _ = String::new();
    } else {
        println!("{}", input);
    }
}


/// Param string
/// Return, VecString or Error
/// Prend un param (ou pas) et renvoie les fichiers du fichier courant
pub fn ls_function(input: &str) -> Result<Vec<String>, std::io::Error>{
        let mut entries_list = Vec::new(); // On prépare la liste des noms de fichier
        let dir = Path::new(input); // On fait un path à l'aide du param
        if dir.is_dir() {
            if let Ok(entries) = fs::read_dir(dir) { // On utilise la fonction read_dir
                for entry in entries { // Pour chaque fichier
                    if let Ok(entry) = entry {
                        if let Ok(nom_fic) = entry.file_name().into_string() { // On récupère le nom
                            entries_list.push(nom_fic); // On l'ajoute dans la entries_list
                        } else {
                            entries_list.push("Fichier corrompu".to_string()); // Sinon on ajoute une erreur
                        }
                    }
                }
            }
            Ok(entries_list) // En cas de réussite, on retourne la liste de fichiers
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a directory")) // Sinon, on retourne une errer
        }
    }
///Param VecString
/// Return VecString ou Error
/// Cat_function permet de renvoyer le contenu des fichiers demandées dans le cat, potentiellement concaténée
pub fn cat_function(file_names: Vec<String>) -> Result<Vec<String>, std::io::Error> {
    let mut files_contents = Vec::new(); // On crée une variable qui va contenir les noms de fichiers
    for file_name in &file_names { // On passe à travers le param file_names
        let mut file_content = String::new(); // On crée un String qui récupère le contenu du fichier
        let file_path = Path::new(file_name); // On crée un Path à partir du nom de fichier
        let mut file = File::open(file_path)?;  // On ouvre le fichier
        file.read_to_string(&mut file_content)?; // On lit le fichier
        files_contents.push(file_content); // On prend le contenu lu dans file_contents
    }
    Ok(files_contents) // On renvoie les contenus des fichiers
}

