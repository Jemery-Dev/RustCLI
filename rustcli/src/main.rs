    use crate::utils::{cat_function, echo_function, ls_function};
    use std::io;
    use std::io::Error;

    mod utils;

    fn main() {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Impossible de lire l'input");

            // On récupère chaque input avec un split (' ')
            let list_inputs: Vec<String> = input.trim().split(' ').map(String::from).collect();

            // Si jamais y a une commande
            if !list_inputs.is_empty() {
                match list_inputs[0].as_str() { // Match de la commande
                    "echo" => {
                        echo_function(list_inputs[1].as_str());
                    },

                    "pipi" => {
                        println!("pipi commande ?");
                    },

                    "ls" => { // La commande ls affiche tout les dossiers d'un dir
                        let ls_param = if list_inputs.len() > 1 {
                            &list_inputs[1] // On check s'il y a un param
                        } else {
                            "./" // Sinon on prend le dir courant
                        };
                        match ls_function(ls_param){ // On récupère les informations de ls_function
                            Ok(entries) => { // Si jamais ls_function renvoie Ok
                                for entry in entries{ // On passe à travers tout les fichiers
                                    println!("{}", entry); // On affiche le file
                                }
                            }
                            Err(_) => { // Si jamais ça renvoie une erreurech
                                println!("Erreur lors de l'affichage du répertoire"); // On le précise
                            }
                        }
                    }

                    "cat" => { // Cat concatène le contenu des fichiers en param
                        if list_inputs.len() > 1 { // Si jamais y a au moins un param
                            // Pour chaque param, on le Vec cat param
                            let cat_param: Vec<String> = list_inputs[1..].iter().map(|s| s.to_string()).collect();
                            match cat_function(cat_param) { // On passe le vec dans la fonction
                                Ok(contents) => { // Si la fonction est réussi
                                    for content in contents {
                                        println!("{}", content); // On écrit le contenun
                                    }
                                }
                                Err(_) => { // Sinon avec une erreur
                                    println!("Erreur lors de la lecture du fichier"); // On le précise
                                }
                            }
                        } else { // Si pas de param, on précise aussi
                            println!("Aucun fichier spécifié pour la commande cat");
                        }
                    }

                    _ => { // Si aucune des commandes ne correspond
                        println!("Commande {} inconnu au bataillon", list_inputs[0].as_str());
                    }
                }
            } else {
                println!("{}", "");
            }
        }
    }
