use crate::counter;
use crate::errors::GameError;
use crate::player::{Combatant, Player, Poison};
use crate::scoring;
use rand::prelude::*;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};


#[derive(Debug, Clone)] // Qui va permettre de clone le GameConfig pour les tests unitaires
pub struct GameConfig {
    pub name1: String,
    pub name2: String,
    pub vitality: i32,
    pub vitesse: i32,
    pub force: i32,
    pub objectives: usize,
}

impl GameConfig {
    pub fn default_with_names(name1: &str, name2: &str) -> Self {
        GameConfig {
            name1: name1.to_string(),
            name2: name2.to_string(),
            vitality: 50,
            vitesse: 50,
            force: 50,
            objectives: 5,
        }
    }
}


#[derive(Debug, Clone)] 
pub struct TurnResult {
    pub average_score: i32,
    pub objective_details: Vec<ObjectiveDetail>,
}


#[derive(Debug, Clone)]
pub struct ObjectiveDetail {
    pub target: i32,
    pub counter_value: i32,
    pub miss: i32,
    pub score: i32,
}


pub fn generate_objectives(count: usize) -> Result<Vec<i32>, GameError> {
    if count == 0 {
        return Err(GameError::GameLogicError("Nombre d'objectifs = 0".to_string()));
    }
    let mut rng = rand::rng();
    let objectives: Vec<i32> = (0..count).map(|_| rng.random_range(0..=100)).collect();
    log::info!("Objectifs générés : {:?}", objectives);
    Ok(objectives)
}



pub fn play_turn(player: &Player, num_objectives: usize) -> Result<TurnResult, GameError> { 
    println!("Au tour de {}", player);

    let objectives = generate_objectives(num_objectives)?;
    println!("Objectifs : {:?}", objectives);
    println!("Appuyer sur ENTREE pour démarrer le tour..");
    counter::wait_for_enter()?;

    let mut scores: Vec<i32> = Vec::new();
    let mut details: Vec<ObjectiveDetail> = Vec::new();

    for &target in objectives.iter() {
        let result = counter::run_counter(player.get_vitesse(), target)?;

        let score = scoring::compute_full_score(
            result.value,
            target,
            player.get_force(),
            result.miss,
        )?;

        let base = scoring::base_score_from_difference(scoring::circular_difference(
            result.value,
            target,
        ));

        println!(
            "\rObjectif {:>3} : Miss = {} | Compteur = {:>3} // Score = ({} + {}) / {} = {}",
            target,
            result.miss,
            result.value,
            base,
            player.get_force(),
            result.miss + 1,
            score
        );

        scores.push(score);
        details.push(ObjectiveDetail {target,counter_value: result.value,miss: result.miss,score,});
    }

    let average = scoring::calculate_round_average(&scores)?;
    println!("# Fin du tour #");
    println!("Score moyen : {}", average);

    Ok(TurnResult {
        average_score: average,
        objective_details: details,
    })
}


pub fn choose_poison(winner_name: &str, loser_name: &str) -> Result<Poison, GameError> {
    println!(
        "{} choisie quel malus appliquer à {} :",
        winner_name, loser_name
    );
    println!("1: -5 vitesse");
    println!("2: -5 force");
    print!("> ");
    io::stdout().flush().map_err(|e| GameError::InputError(e.to_string()))?;

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).map_err(|e| GameError::InputError(e.to_string()))?;

    match input.trim() {
        "1" => Ok(Poison::Reductionvitesse),
        "2" => Ok(Poison::Reductionforce),
        _ => { 
            println!("Entrée invalide. Veuillez choisir 1 ou 2 et PLUS VITE QUE CA.");
            choose_poison(winner_name, loser_name)
        }
    }
}


pub fn ask_replay() -> Result<bool, GameError> {
    println!("Relancer une partie ? [Y/N]");
    print!("> ");
    io::stdout().flush().map_err(|e| GameError::InputError(e.to_string()))?;

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).map_err(|e| GameError::InputError(e.to_string()))?;

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" | "o" | "oui" => Ok(true),
        _ => Ok(false),
    }
}


pub fn run_game(config: &GameConfig) -> Result<(), GameError> {
    loop {
        let mut player1 =
            Player::new(&config.name1, config.vitality, config.vitesse, config.force)?;
        let mut player2 =
            Player::new(&config.name2, config.vitality, config.vitesse, config.force)?;

        println!("##### Démarrage de la partie LETSS GOOOO#####");
        let mut round: u32 = 1;

        loop {
            println!("## Manche {} ##", round);

            let result1 = play_turn(&player1, config.objectives)?;
            let result2 = play_turn(&player2, config.objectives)?;

            let score1 = result1.average_score;
            let score2 = result2.average_score;

            if score1 > score2 {
                let damage = score1 - score2;
                println!(
                    "{} gagne la manche. {} perd {} points de vitalité.",
                    player1.get_name(), player2.get_name(), damage
                );
                player2.take_damage(damage)?;
                if player2.is_alive() {
                    let poison = choose_poison(player1.get_name(), player2.get_name())?;
                    player2.apply_poison(poison);
                }
            } else if score2 > score1 {
                let damage = score2 - score1;
                println!(
                    "{} gagne la manche. {} perd {} points de vitalité.",
                    player2.get_name(), player1.get_name(), damage
                );
                player1.take_damage(damage)?;
                if player1.is_alive() {
                    let poison = choose_poison(player2.get_name(), player1.get_name())?;
                    player1.apply_poison(poison);
                }
            } else {
                println!("Egalité !!! Rien ne se passe.");
            }

            println!("## FIN de la manche {} ##", round);

            if !player1.is_alive() {
                println!("##### Partie terminée #####");
                println!("{} a été tué. {} gagne !", player1.get_name(), player2.get_name());
                break;
            }
            if !player2.is_alive() {
                println!("##### Partie terminée #####");
                println!("{} a été tué. {} gagne !", player2.get_name(), player1.get_name());
                break;
            }

            round += 1;
        }

        if !ask_replay()? {
           
            break;
        }
    }

    Ok(())
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_objectives_valid() {
        let obj = generate_objectives(5).unwrap();
        assert_eq!(obj.len(), 5);
        for &v in &obj {
            assert!(v >= 0 && v <= 100);
        }
    }

    #[test]
    fn test_generate_objectives_zero() {
        assert!(generate_objectives(0).is_err());
    }

    #[test]
    fn test_game_config_default() {
        let cfg = GameConfig::default_with_names("Michel", "Jacque");
        assert_eq!(cfg.name1, "Michel");
        assert_eq!(cfg.name2, "Jacque");
        assert_eq!(cfg.vitality, 50);
        assert_eq!(cfg.vitesse, 50);
        assert_eq!(cfg.force, 50);
        assert_eq!(cfg.objectives, 5);
    }
}
