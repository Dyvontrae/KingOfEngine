// Declarations

use rand::Rng; // This "unlocks" gen_range for your rng variable

/// Represents the six possible outcomes of a single die roll.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DieResult {
    One,
    Two,
    Three,
    Energy,     // In-game currency
    Claw,       // Attack/Tokyo
    Heart,      // +1 HP
}

// -- Dice Roll Implementation -- The mechanics of roll only, handling rerolls and phases is handled in both primary game logic and the main game loop. 

/// Performs a full roll of all 6 dice and returns the results.
pub fn roll_dice() -> [DieResult; 6] {
    let mut rng = rand::thread_rng();
    let mut results = [DieResult::One; 6];

    for i in 0..6 {
        let roll = rng.gen_range(1..=6);
        results[i] = match roll {
            1 => DieResult::One,
            2 => DieResult::Two,
            3 => DieResult::Three,
            4 => DieResult::Energy,
            5 => DieResult::Claw,
            6 => DieResult::Heart,
            _ => unreachable!(),
        };
    }
    results
}

/// Rerolls the dice at the given 0-based indices. This was the missing function!
pub fn reroll_dice(results: &mut [DieResult; 6], indices_to_reroll: &[usize]) {
    let mut rng = rand::thread_rng();
    
    for &index in indices_to_reroll {
        if index < 6 {
            let roll = rng.gen_range(1..=6);
            results[index] = match roll {
                1 => DieResult::One,
                2 => DieResult::Two,
                3 => DieResult::Three,
                4 => DieResult::Energy,
                5 => DieResult::Claw,
                6 => DieResult::Heart,
                _ => unreachable!(),
            };
        }
    }
}

 