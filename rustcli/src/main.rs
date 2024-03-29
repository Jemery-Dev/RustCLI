    use crate::utils::{echo_function, ls_function};
    use std::io;
    use std::io::Error;

    mod utils;

    fn main() {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Impossible de lire l'input");

            let list_inputs: Vec<String> = input.trim().split(' ').map(String::from).collect();

            if !list_inputs.is_empty() {
                match list_inputs[0].as_str() {
                    "echo" => {
                        echo_function(list_inputs[1].as_str());
                    },

                    "pipi" => {
                        println!("pipi commande ?");
                    },

                    "ls" => {
                        let ls_param = if list_inputs.len() > 1 {
                            &list_inputs[1]
                        } else {
                            "../fichier_test/"
                        };
                        match ls_function(ls_param){
                            Ok(entries) => {
                                for entry in entries{
                                    println!("{}", entry);
                                }
                            }
                            Err(_) => {
                                println!("Erreur lors de l'affichage du répertoire");
                            }
                        }
                    }

                    _ => {
                        println!("Commande {} inconnu au bataillon", list_inputs[0].as_str());
                    }
                }
            } else {
                println!("{}", "");
            }
        }
    }
