use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    //    let nombre_aleatoire = rand::thread_rng().gen_range(1..=100);

    let mut rng = rand::rng();
    let nombre_aleatoire = rng.random_range(1..=100);

    println!("Devinez le nombre!");

    loop {
        println!("Veuillez entrer un nombre.");

        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Echec de la lecture de l'entrée utilisateur");

        let saisie: u32 = match saisie.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue;
            }
        };

        match saisie.cmp(&nombre_aleatoire) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
