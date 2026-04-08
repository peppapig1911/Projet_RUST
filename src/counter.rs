use crate::errors::GameError;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct CounterState {
    pub value: i32,
    pub miss: i32,
    pub running: bool,
    pub stopped: bool,
}

impl CounterState {
    pub fn new() -> Self {
        CounterState {
            value: 0,
            miss: 0,
            running: false,
            stopped: false,
        }
    }

    pub fn reset(&mut self) {
        self.value = 0;
        self.miss = 0;
        self.running = true;
        self.stopped = false;
    }
}


#[derive(Debug, Clone, Copy)]
pub struct CounterResult {
    pub value: i32,
    pub miss: i32,
}

pub fn run_counter(vitesse_ms: i32, target: i32) -> Result<CounterResult, GameError> {
    let state = Arc::new(Mutex::new(CounterState::new()));
    {
        let mut s = state.lock().map_err(|e| GameError::GameLogicError(format!("Erreur mutex: {}", e)))?;
        s.reset();
    }
    let state_increment = Arc::clone(&state);
    let vitesse = vitesse_ms as u64;

    let increment_handle = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(vitesse));
            let mut s = match state_increment.lock() {
                Ok(guard) => guard,
                Err(_) => break,
            };
            if s.stopped {
                break;
            }
            s.value += 1;
            if s.value > 100 {
                s.value = 0;
                s.miss += 1;
                log::trace!("Tour complet, miss = {}", s.miss);
            }
        }
    });


    let state_display = Arc::clone(&state);
    let display_handle = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(30));
            let s = match state_display.lock() {
                Ok(guard) => guard,
                Err(_) => break,
            };
            if s.stopped {
                break;
            }
            print!(
                "\r Objectif {:>3} : Miss = {} | Compteur = {:>3}  ",
                target, s.miss, s.value
            );
            use std::io::Write;
            let _ = io::stdout().flush();
        }
    });


    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .map_err(|e| GameError::InputError(format!("Erreur stdin: {}", e)))?;

    let result = {
        let mut s = state.lock().map_err(|e| GameError::GameLogicError(format!("Erreur mutex: {}", e)))?;
        s.stopped = true;
        s.running = false;
        CounterResult {value: s.value, miss: s.miss,}
    };

    let _ = increment_handle.join();
    let _ = display_handle.join();

    Ok(result)
}


pub fn wait_for_enter() -> Result<(), GameError> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).map_err(|e| GameError::InputError(format!("Erreur stdin: {}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_state_reset() { // Test qui vérifie que la méthode reset() de CounterState réinitialise correctement les valeurs du compteur, du nombre de misses, et les flags de running et stopped.
        let mut state = CounterState::new();
        state.value = 50;
        state.miss = 3;
        state.stopped = true;
        state.reset();
        assert_eq!(state.value, 0);
        assert_eq!(state.miss, 0);
        assert!(state.running);
        assert!(!state.stopped);
    }
   

    #[test]
    fn test_counter_result() { // Test qui vérifie que la structure CounterResult peut être créée et contient les valeurs correctes pour le compteur et le nombre de misses.
        let result = CounterResult { value: 36, miss: 1 };
        assert_eq!(result.value, 36);
        assert_eq!(result.miss, 1);
    }

    #[test]
    fn test_increment_simulation() { // Test qui simule l'incrémentation du compteur dans un thread et vérifie que les valeurs du compteur et des misses sont mises à jour correctement après un certain nombre d'incréments.
        let state = Arc::new(Mutex::new(CounterState::new()));
        {
            let mut s = state.lock().unwrap();
            s.reset();
        }

        let state_inc = Arc::clone(&state);
        let handle = thread::spawn(move || {
            for _ in 0..105 {
                let mut s = state_inc.lock().unwrap();
                if s.stopped {
                    break;
                }
                s.value += 1;
                if s.value > 100 {
                    s.value = 0;
                    s.miss += 1;
                }
            }
        });

        handle.join().unwrap();

        let s = state.lock().unwrap();
        // 105 incréments : 1 tour (101) + 4 => value=4, miss=1
        assert_eq!(s.value, 4);
        assert_eq!(s.miss, 1);
    }

    #[test]
    fn test_stop_flag() { // Test qui vérifie que le drapeau stop fonctionne correctement.  
        let state = Arc::new(Mutex::new(CounterState::new()));
        {
            let mut s = state.lock().unwrap();
            s.reset();
        }

        let state_inc = Arc::clone(&state);
        let state_main = Arc::clone(&state);

        let handle = thread::spawn(move || {
            loop {
                let mut s = state_inc.lock().unwrap();
                if s.stopped {
                    break;
                }
                s.value += 1;
                if s.value > 100 {
                    s.value = 0;
                    s.miss += 1;
                }
                drop(s);
                thread::sleep(Duration::from_millis(1));
            }
        });

        thread::sleep(Duration::from_millis(50));
        {
            let mut s = state_main.lock().unwrap();
            s.stopped = true;
        }

        handle.join().unwrap();
        let s = state.lock().unwrap();
        assert!(s.stopped);
        assert!(s.value >= 0 && s.value <= 100);
    }
}
