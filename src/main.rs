use clap::Parser;
use duel_game::game::{self, GameConfig};
use log::error;

#[derive(Parser, Debug)]
#[command(name = "Duel Game", version, about = "Jeu de duel")]
struct Args {
    
    #[arg( long = "name1", default_value = "Toto")] 
    name1: String,

    #[arg( long = "name2", default_value = "Titi")]
    name2: String,

    #[arg(long, default_value_t = 50)]
    vitality: i32,

    #[arg( long, default_value_t = 5)]
    objectifs: usize,

    #[arg( long, default_value_t = 50)]
    vitesse: i32,

    #[arg(long, default_value_t = 50)]
    force: i32,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    log::info!(
        "Lancement : {} vs {} | V={} S={} F={} O={}",
        args.name1, args.name2, args.vitality, args.vitesse, args.force, args.objectifs
    );

    let config = GameConfig {
        name1: args.name1,
        name2: args.name2,
        vitality: args.vitality,
        vitesse: args.vitesse,
        force: args.force,
        objectives: args.objectifs,
    };

    match game::run_game(&config) {
        Ok(()) => log::info!("Partie terminée"),
        Err(e) => {
            error!("Erreur fatale : {}", e);
            eprintln!("Erreur fatale : {}", e);
            std::process::exit(1);
        }
    }
}
