use std::io;

fn main() {
    println!("Devinez le nombre!");

    println!("Veuillez entrer votre nombre!");

    let mut saisie = String::new();

    io::stdin()
        .read_line(&mut saisie)
        .expect("Echec de la lecture de l'entrée utilisateur");

    println!("Vous avez entré: {}", saisie);
}
