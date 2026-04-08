use crate::errors::GameError;
use std::fmt;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Poison {
    Reductionvitesse,
    Reductionforce,
}


pub trait Combatant {
    fn get_name(&self) -> &str;
    fn get_vitality(&self) -> i32;
    fn get_vitesse(&self) -> i32;
    fn get_force(&self) -> i32;
    fn is_alive(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    vitality: i32,
    vitesse: i32,
    force: i32,
}

impl Player {
    pub fn new(name: &str, vitality: i32, vitesse: i32, force: i32) -> Result<Self, GameError> {
        if name.is_empty() {
            return Err(GameError::PlayerError("Le nom ne peut pas être vide".to_string()));
        }
        if vitality <= 0 {
            return Err(GameError::PlayerError("La vitalité doit être positive".to_string()));
        }
        if vitesse <= 0 {
            return Err(GameError::PlayerError("La vitesse doit être positive".to_string()));
        }
        if force < 0 {
            return Err(GameError::PlayerError("La force ne peut pas être négative".to_string()));
        }
        Ok(Player {
            name: name.to_string(),
            vitality,
            vitesse,
            force,
        })
    }


    pub fn take_damage(&mut self, damage: i32) -> Result<(), GameError> {
        if damage < 0 {
            return Err(GameError::PlayerError("Les dégâts ne peuvent pas être négatifs".to_string()));
        }
        self.vitality = (self.vitality - damage).max(0);
        log::info!("{} subit {} dégâts. Vitalité restante: {}", self.name, damage, self.vitality);
        Ok(())
    }


    pub fn apply_poison(&mut self, poison: Poison) {
        match poison {
            Poison::Reductionvitesse => {
                self.vitesse = (self.vitesse - 5).max(1);
                log::info!("{} perd 5 de vitesse. vitesse: {}", self.name, self.vitesse);
            }
            Poison::Reductionforce => {
                self.force = (self.force - 5).max(0);
                log::info!("{} perd 5 de force. force: {}", self.name, self.force);
            }
        }
    }


    pub fn reset(&mut self, vitality: i32, vitesse: i32, force: i32) {
        self.vitality = vitality;
        self.vitesse = vitesse;
        self.force = force;
    }
}

impl Combatant for Player {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_vitality(&self) -> i32 {
        self.vitality
    }
    fn get_vitesse(&self) -> i32 {
        self.vitesse
    }
    fn get_force(&self) -> i32 {
        self.force
    }
    fn is_alive(&self) -> bool {
        self.vitality > 0
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (Vitality={}, vitesse={}, force={})",
            self.name, self.vitality, self.vitesse, self.force
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_player_valid() {
        let player = Player::new("Tommy", 50, 50, 50);
        assert!(player.is_ok());
        let p = player.unwrap();
        assert_eq!(p.get_name(), "Tommy");
        assert_eq!(p.get_vitality(), 50);
    }

    #[test]
    fn test_new_player_empty_name() {
        assert!(Player::new("", 50, 50, 50).is_err());
    }

    #[test]
    fn test_new_player_negative_vitality() {
        assert!(Player::new("Test", -10, 50, 50).is_err());
    }

    #[test]
    fn test_new_player_zero_vitesse() {
        assert!(Player::new("Test", 50, 0, 50).is_err());
    }

    #[test]
    fn test_take_damage() {
        let mut p = Player::new("Tommy", 50, 50, 50).unwrap();
        assert!(p.take_damage(13).is_ok());
        assert_eq!(p.get_vitality(), 37);
    }

    #[test]
    fn test_poison_vitesse() {
        let mut p = Player::new("Tommy", 50, 50, 50).unwrap();
        p.apply_poison(Poison::Reductionvitesse);
        assert_eq!(p.get_vitesse(), 45);
    }

    #[test]
    fn test_poison_force() {
        let mut p = Player::new("Tommy", 50, 50, 50).unwrap();
        p.apply_poison(Poison::Reductionforce);
        assert_eq!(p.get_force(), 45);
    }

    #[test]
    fn test_vivant() {
        let p = Player::new("Tommy", 50, 50, 50).unwrap();
        assert!(p.is_alive());
    }

    #[test]
    fn test_mort() {
        let mut p = Player::new("Tommy", 10, 50, 50).unwrap();
        let _ = p.take_damage(10);
        assert!(!p.is_alive());
    }
}
