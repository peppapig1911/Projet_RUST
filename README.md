# Duel Game
## Lancer le jeu

```bash
cargo run -- --name1 Tommy --name2 Christine --vitality 50 --objectifs 5
```

## Arguments

- `--name1`  : nom du joueur 1 
- `--name2`  : nom du joueur 2 
- `--vitality` : points de vie par défaut c'est 50
- `--speed` : vitesse en ms par défaut c'est 50
- `--force` : force par defaut 50 aussi
- `--objectifs`  : nombre d'objectifs par tour par defaut c'est 5

## Tests

```bash
cargo test
```

## Logs

```bash
RUST_LOG=info cargo run -- --name1 Michel --name2 Jacque
```

## Dépendances

- clap : arguments CLI
- rand : génération aléatoire
- log + env_logger : logging
