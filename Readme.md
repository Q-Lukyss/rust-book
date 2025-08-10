# Exercices Rust

![Rust Version](https://img.shields.io/badge/Rust-1.88-orange?logo=rust&logoColor=white&style=for-the-badge)

Ce repo regroupe les exercices et programmes que j'ai réalisé dans le cadre de mon apprentissage de [Rust](https://www.rust-lang.org/) via le [RustBook](https://doc.rust-lang.org/stable/book/)    

## Lancer un programme

```
cd <Nom du dossier>
cargo run
```

### test-in-rust
```
cargo test
```

### minigrep

#### Powershell

Lancer avec IGNORE_CASE

```
$Env:IGNORE_CASE=1; cargo run -- to poem.txt
```
L'Enlever
```
Remove-Item Env:IGNORE_CASE
```
#### Linux
```
IGNORE_CASE=1 cargo run -- to poem.txt
```