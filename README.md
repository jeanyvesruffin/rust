# rust

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
