# rust

* Documentation rust

[https://doc.rust-lang.org/std/index.html](https://doc.rust-lang.org/std/index.html)

* Cargo repository
  
[https://crates.io/](https://crates.io/)

## Ligne de commandes rust

* Accès documentation

```sh
rustup docs
```

* Voir la version

```sh
rustup --version
# rustc 1.96.0 (ac68faa20 2026-05-25)
```

* Mettre à jour rust
  
```sh
rustup update
# info: syncing channel updates for stable-x86_64-pc-windows-msvc
#  stable-x86_64-pc-windows-msvc unchanged - rustc 1.96.0 (ac68faa20 2026-05-25)
# info: checking for self-update (current version: 1.29.0)
# info: cleaning up downloads & tmp directories
```

* Désinstaller rust
  
```sh
rustup self uninstall
```

## Hello world

### Classe main (\rust\Hello_World\main.rs)

```rs
fn main(){
    println!("Hello, world!");
}
```

### Compilation de main.rs (\rust\Hello_World\main.rs)

```rs
rustc main.rs
```

Cela créé les ficheirs suivants :

```rs
main.exe
main.pdb
main.rs
```

### Exécuter le programme (\rust\Hello_World\main.exe)

```rs
.\main.exe
```

## Cargo

_Cargo est le système de compilation et de gestion de paquets de Rust_

## Ligne de commandes Cargo

* Mise à jours des package de cargo

```sh
cargo update
```

* Ajouter un package à cargo

```sh
cargo add <package>
```

* Génération de la documentation

```sh
cargo doc --open
```

* Accès documentation

```sh
cargo --help
```

* Voir la version

```sh
cargo --version
# cargo 1.96.0 (30a34c682 2026-05-25)
```

* Créer un projet avec Cargo

```sh
cargo new hello_cargo
#    Creating binary (application) `hello_cargo` package
# note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
cd .\hello_cargo\
```

Fichier : Cargo.toml

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
Encart 1-2 : Contenu de Cargo.toml généré par cargo new
```

Ce fichier est au format TOML (Tom’s Obvious, Minimal Language), qui est le format de configuration de Cargo.

* Compiler et exécuter un projet Cargo

```sh
cd .\hello_cargo\
cargo build
#   Compiling hello_cargo v0.1.0 (C:\Users\ruffi\WORKSPACE\rust\hello_cargo)
#    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
.\target\debug\hello_cargo.exe
# Hello, world!
# ou alors build + execute
cargo run
#    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
#     Running `target\debug\hello_cargo.exe`
# Hello, world!
```

* Vérification compilation

```sh
cargo check
#  Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
```

* Création d'une nouvelle release (mise en production)

```sh
cargo build --release
#     Compiling hello_cargo v0.1.0 (C:\Users\ruffi\WORKSPACE\rust\hello_cargo)
#    Finished `release` profile [optimized] target(s) in 0.35s
```

## Program_1

Nous allons coder un programme fréquemment réalisé par les débutants en programmation : le jeu du plus ou du moins. Le principe de ce jeu est le suivant : le programme va tirer au sort un nombre entre 1 et 100. Il invitera ensuite le joueur à saisir un nombre qu'il pense deviner. Après la saisie, le programme indiquera si le nombre saisi par le joueur est trop grand ou trop petit. Si le nombre saisi est le bon, le jeu affichera un message de félicitations et se fermera.

### Declarer une variable

```rs
let mut saisie = String::new();
```

la ligne `let mut saisie = String::new();` permet de créer une variable mutable nommée `saisie`. Le signe égal (`=`) indique à Rust que nous voulons désormais lier quelquechose à la variable. A la droite du signe égal, nous avons la valeur liée à `saisie`, qui est ici le résultat de l'utilisation de `String::new`, qui est une fonction qui retourne une nouvelle instance de `String`. `String` est un type de chaîne de caractères fourni par la bibliothèque standard, qui est une portion de texte encodée en UTF-8 et dont la longueur peut augmenter.

La syntaxe `::` dans `String::new()` indique que new est une **fonction associée au type String**. Une fonction associée est une fonction qui est implémentée sur un type, ici String. Cette fonction new crée une nouvelle chaîne de caractères vide, une nouvelle String. Vous trouverez fréquemment une fonction new sur d'autres types, car c'est un nom souvent donné à une fonction qui crée une nouvelle valeur ou instance d'un type.

### Recueillir la saisie utilisateur

```rs
    io::stdin()
        .read_line(&mut saisie)
```

Si nous n'avions pas importé la bibliothèque `io` avec `use std::io` au début du programme, on aurait toujours pu utiliser la fonction en écrivant l'appel à la fonction de cette manière : `std::io::stdin`. La fonction `stdin` retourne une instance de `std::io::Stdin`, qui est un type qui représente une référence abstraite (handle) vers l'entrée standard du terminal dans lequel vous avez lancé le programme.

Ensuite, la ligne `.read_line(&mut saisie)` appelle la méthode `read_line` sur l'entrée standard afin d'obtenir la saisie utilisateur. Nous passons aussi `&mut saisie` en argument de `read_line` pour lui indiquer dans quelle chaîne de caractère il faut stocker la saisie utilisateur. Le but final de `read_line` est de récupérer tout ce que l'utilisateur écrit dans l'entrée standard et de l'ajouter à la fin d'une chaîne de caractères (sans écraser son contenu) ; c'est pourquoi nous passons cette chaîne de caractères en argument. Cet argument doit être mutable pour que `read_line` puisse en modifier le contenu.

Le `&` indique que cet argument est **une référence**, ce qui permet de laisser plusieurs morceaux de votre code accéder à une même donnée sans avoir besoin de copier ces données dans la mémoire plusieurs fois. Les références sont une fonctionnalité complexe, et un des avantages majeurs de Rust est qu'il rend sûr et simple l'utilisation des références. Il n'est pas nécessaire de trop s'apesantir sur les références pour terminer ce programme. Pour l'instant, tout ce que vous devez savoir est que _comme les variables, les références sont immuables par défaut_. D'où la nécessité d'écrire `&mut saisie` au lieu de `&saisie` pour la rendre mutable.

### Gérer les erreurs potentielles avec le type Result

```rs
 .expect("Échec de la lecture de l'entrée utilisateur");
```

Comme expliqué précédemment, `read_line` stocke dans la variable qu'on lui passe en argument tout ce que l'utilisateur a saisi, mais cette fonction retourne aussi une valeur − dans notre cas, de type `io::Result`. Il existe plusieurs types nommés `Result` dans la bibliothèque standard de Rust : un type générique `Result` ainsi que des déclinaisons spécifiques à des sous-modules, comme `io::Result`. Les **types Result sont des énumérations**, aussi appelées `enums`, qui peuvent avoir un certain nombre de valeurs prédéfinies que l'on appelle variantes. Les énumérations sont souvent utilisées avec match, une structure conditionelle qui facilite l'exécution d'un code différent en fonction de la variante dans l'énumération au moment de son évaluation.

Les variantes de `Result` sont `Ok` et `Err`. La variante `Ok` signifie que l'opération a fonctionné, et à l'intérieur de Ok se trouve la valeur générée avec succès. La variante `Err` signifie que l'opération a échoué, et Err contient les informations décrivant comment ou pourquoi l'opération a échoué.

Les valeurs du type Result, comme pour tous les types, ont des méthodes qui leur sont associées. Par exemple, une instance de `io::Result` a une méthode `expect` que vous pouvez utiliser. Si cette instance de io::Result a pour valeur la variante Err, l'appel à expect fera planter le programme et affichera le message que vous avez passé en argument de expect. Si l'appel à read_line retourne une variante Err, ce sera probablement dû à une erreur du système d'exploitation. Si en revanche read_line a pour valeur la variante Ok, expect récupèrera le contenu du Ok, qui est le résultat de l'opération, et vous le retournera afin que vous puissiez l'utiliser. Dans notre exemple, ce résultat est le nombre d'octets de la saisie utilisateur.

### Étendre les fonctionnalités de Rust avec une **crate**

Souvenez-vous, une crate est un ensemble de fichiers de code source Rust. Le projet sur lequel nous travaillons est une crate binaire, qui est un programme exécutable. La crate rand est une crate de bibliothèque, qui contient du code qui peut être utilisé dans d'autres programmes, et qui ne peut pas être exécuté tout seul.

La coordination des crates externes est un domaine dans lequel Cargo excelle. Avant d'écrire le code qui utilisera rand, il nous faut éditer le fichier Cargo.toml pour y spécifier rand en tant que dépendance. Ouvrez donc maintenant ce fichier et ajoutez la ligne suivante à la fin, en dessous de l'en-tête de section [dependencies] que Cargo a créé pour vous. Assurez-vous de spécifier rand exactement comme dans le bout de code suivant, avec ce numéro de version, ou sinon les exemples de code de ce tutoriel pourraient ne pas fonctionner.

```toml
rand = "0.8.3"
```

### Générer un nombre aléatoire

```rs
use rand::prelude::*;
////
    let mut rng = rand::rng();
    let nombre_aleatoire = rng.random_range(1..=100);
```

### Convertie typage

*Convertie String en ui32 (Le type entier non signé 32 bits.)

```rs
  let saisie: u32 = saisie.trim().parse().expect("Veuillez entrer un nombre !");
```

### Comparer le nombre saisi au nombre secret

```rs
use std::cmp::Ordering;
      match saisie.cmp(&nombre_aleatoire) {
        Ordering::Less => println!("C'est plus !"),
        Ordering::Greater => println!("C'est moins !"),
        Ordering::Equal => println!("Vous avez gagné !"),
    }
```

### Ajout d'une boucle

```rs
loop {
}
```

### Gérer les saisies invalides

```rs
let saisie: u32 = match saisie.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue;
            }
        };
```
