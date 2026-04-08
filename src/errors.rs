use std::fmt;

#[derive(Debug)]
pub enum GameError {
    InputError(String),
    PlayerError(String),
    GameLogicError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InputError(msg) => write!(f, "Erreur Input: {}", msg),
            GameError::PlayerError(msg) => write!(f, "Erreur Player: {}", msg),
            GameError::GameLogicError(msg) => write!(f, "Erreur game: {}", msg),
        }
    }
}

impl std::error::Error for GameError {}

impl From<std::io::Error> for GameError {
    fn from(err: std::io::Error) -> Self {
        GameError::InputError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_error_display() { // Test qui vérifie que le message d'erreur d'une InputError est correctement formaté lorsqu'on utilise la méthode to_string() ou format!() pour afficher l'erreur.
        let err = GameError::InputError("touche invalide".to_string());
        assert_eq!(format!("{}", err), "Erreur Input: touche pas valide");
    }

    #[test]
    fn test_player_error_display() { // Test qui vérifie que le message d'erreur d'une PlayerError est correctement formaté lorsqu'on utilise la méthode to_string() ou format!() pour afficher l'erreur.
        let err = GameError::PlayerError("vitalité insuffisante".to_string());
        assert_eq!(format!("{}", err), "Erreur Player: vitalité insuffisante");
    }

    #[test]
    fn test_game_logic_error_display() { // Test qui vérifie que le message d'erreur d'une GameLogicError est correctement formaté lorsqu'on utilise la méthode to_string() ou format!() pour afficher l'erreur.
        let err = GameError::GameLogicError("thread panic".to_string());
        assert_eq!(format!("{}", err), "Erreur game: thread panique");
    }

    #[test]
    fn test_from_io_error() { // Test qui vérifie que la conversion d'une erreur std::io::Error en GameError fonctionne correctement et que le message d'erreur est correctement encapsulé dans un GameError::InputError.
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "fichier absent");
        let game_err: GameError = GameError::from(io_err);
        match game_err {
            GameError::InputError(msg) => assert!(msg.contains("fichier absent")),
            _ => panic!("Mauvais type d'erreur"),
        }
    }
}
