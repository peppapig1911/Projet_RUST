use crate::errors::GameError;

// Le score est en cercle et on fait la difference 
pub fn circular_difference(value: i32, target: i32) -> i32 {
    let diff = (value - target).abs();
    if diff > 50 {
        101 - diff
    } else {
        diff
    }
}


pub fn base_score_from_difference(difference: i32) -> i32 {
    match difference {
        0 => 100,
        1..=5 => 80,
        6..=10 => 60,
        11..=20 => 40,
        21..=40 => 20,
        _ => 0,
    }
}
pub fn calculate_objective_score(
    base_score: i32,
    force: i32,
    miss: i32,
) -> Result<i32, GameError> {
    if miss < 0 {
        return Err(GameError::GameLogicError("Miss négatif".to_string()));
    }
    Ok((base_score + force) / (miss + 1))
}


pub fn calculate_round_average(scores: &[i32]) -> Result<i32, GameError> {
    if scores.is_empty() {
        return Err(GameError::GameLogicError("Tableau de scores vide".to_string()));
    }
    let sum: i32 = scores.iter().sum();
    let count = scores.len() as f64;
    Ok((sum as f64 / count).ceil() as i32)
}


pub fn compute_full_score(
    counter_value: i32,
    target: i32,
    force: i32,
    miss: i32,
) -> Result<i32, GameError> {
    let diff = circular_difference(counter_value, target);
    let base = base_score_from_difference(diff);
    let score = calculate_objective_score(base, force, miss)?;
    log::debug!(
        "Obj {} : Compteur={}, Diff={}, Base={}, Miss={}, Score={}",
        target, counter_value, diff, base, miss, score
    );
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_diff_zero() {
        assert_eq!(circular_difference(50, 50), 0);
    }

    #[test]
    fn test_circular_diff_wrap() {
        // 95 -> 100 -> 0 -> 15 = 5 + 16 = 21 
        assert_eq!(circular_difference(95, 15), 21);
    }

    #[test]
    fn test_circular_diff_extremes() { 
        assert_eq!(circular_difference(0, 100), 1);
        assert_eq!(circular_difference(100, 0), 1);
    }

    #[test]
    fn test_base_score_perfect() { 
        assert_eq!(base_score_from_difference(0), 100);
    }

    #[test]
    fn test_objective_score_no_miss() {
        let score = calculate_objective_score(80, 50, 0).unwrap();
        assert_eq!(score, 130);
    }

    #[test]
    fn test_objective_score_with_miss() {
        let score = calculate_objective_score(40, 50, 1).unwrap();
        assert_eq!(score, 45);
    }

    #[test]
    fn test_objective_score_negative_miss() {
        assert!(calculate_objective_score(80, 50, -1).is_err());
    }

    #[test]
    fn test_round_average() {
        let scores = vec![45, 130, 130, 55, 65];
        assert_eq!(calculate_round_average(&scores).unwrap(), 85);
    }


    #[test]
    fn test_round_average_empty() {
        let scores: Vec<i32> = vec![];
        assert!(calculate_round_average(&scores).is_err());
    }

    #[test]
    fn test_full_score() { 
        assert_eq!(compute_full_score(80, 82, 50, 0).unwrap(), 130);
    }

    #[test]
    fn test_full_score_with_miss() {
        assert_eq!(compute_full_score(36, 50, 50, 1).unwrap(), 45);
    }
}
