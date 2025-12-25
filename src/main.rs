// -- Calling all core game function libraries and character information for main game loop --
use king_core::game::Game;
use king_core::dice::{roll_dice, reroll_dice, DieResult};
use std::io::{self, Write};

//-- Helper for the main loop since handle_reroll uses IO
fn read_line_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

/// Helper for reroll logic in the main loop
// src/main.rs

fn handle_reroll(initial_results: [DieResult; 6], max_rerolls: u8) -> [DieResult; 6] {
    let mut current_results = initial_results;
    for i in 0..max_rerolls {
        println!("\n    🎲 Current Dice: {:?}", current_results);
        let prompt = format!("    ❓ Reroll {} of {}: (Indices 1-6, 0 for all, X to keep): ", i + 1, max_rerolls);
        let input = read_line_input(&prompt);
        
        // 1. Use the parser from your king_core library
        match king_core::parser::parse_reroll_input(&input) {
            Ok(indices) => {
                // 2. If the user typed 'X', the parser returns an empty Vec
                if indices.is_empty() && input.to_lowercase() == "x" { 
                    break; 
                }
                
                // 3. Only reroll the specific indices returned by the parser
                king_core::dice::reroll_dice(&mut current_results, &indices);
            }
            Err(e) => {
                println!("    ⚠️ {}", e);
                // Optional: decrement 'i' if you want to give them another try on the same reroll phase
            }
        }
    }
    current_results
}

fn main() {
    println!("# 🦖 KING OF TOKYO ENGINE 🏙️ #");
    
    let num_players_str = read_line_input("How many players (2-6)? ");
    let num_players: usize = num_players_str.parse().unwrap_or(2).min(6).max(2);
    
    let mut player_names = Vec::new();
    for i in 0..num_players {
        player_names.push(read_line_input(&format!("Enter name for Player {}: ", i + 1)));
    }
    
    let player_refs: Vec<&str> = player_names.iter().map(|s| s.as_str()).collect();
    let mut game = Game::new(&player_refs);
    
    let mut turn_count = 1;
    let mut current_player_index = 0;

    loop {
        current_player_index %= game.players.len(); 
        let current_player_id = game.players[current_player_index].id;
        
        if game.players[current_player_index].hp == 0 {
            current_player_index += 1;
            continue;
        }

        println!("\n--- Turn {} - {}'s Turn ---", turn_count, game.players[current_player_index].name);
        
        game.apply_tokyo_control_points();

        if let Some(msg) = game.check_victory_condition() {
            println!("\n🎉 {}", msg);
            break;
        }

        let final_results = handle_reroll(roll_dice(), 2);
        game.process_roll(current_player_id, &final_results);
        
        if let Some(msg) = game.check_victory_condition() {
            println!("\n🎉 {}", msg);
            break;
        }

        current_player_index += 1;
        turn_count += 1;
    }
}