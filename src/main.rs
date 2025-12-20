use rand::Rng;
use std::collections::HashMap;
use std::io::{self, Write};

// --- 1. Core Data Structures ---

/// Represents the six possible outcomes of a single die roll.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum DieResult {
    One,
    Two,
    Three,
    Energy,     // In-game currency
    Claw,       // Attack/Tokyo
    Heart,      // +1 HP
}

/// Represents a single Kaiju player's state.
#[derive(Debug)]
struct Player {
    id: u32,
    name: String,
    hp: u8,         // Max 12, start 10
    victory_points: u8, // Max 20
    energy: u8,     // Currency
}

impl Player {
    fn new(id: u32, name: &str) -> Self {
        Player {
            id,
            name: name.to_string(),
            hp: 10, // Start HP
            victory_points: 0,
            energy: 0,
        }
    }
}

/// The central Game manager.
struct Game {
    players: Vec<Player>,
    tokyo_controller_id: Option<u32>, // ID of the player currently in Tokyo (or None)
    max_hp: u8,
    max_vp: u8,
}

// --- Helper Function for Reading Input ---

fn read_line_input(prompt: &str) -> String {
    print!("{}", prompt);
    // Flush the output buffer to ensure the prompt is displayed before input
    io::stdout().flush().expect("Failed to flush stdout"); 
    let mut input = String::new();
    // Use read_line to capture the input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// -- Cleans up Input for rolls and reroll information; parser

fn parse_reroll_input(input: &str) -> Result<Vec<usize>, String> {
    let lower_input = input.trim().to_lowercase();

    if lower_input == "x" {
        return Ok(Vec::new()); // 'X' means cancel, so an empty list of indices
    }
    if lower_input == "0" {
        // '0' means reroll all dice (indices 0 through 5)
        return Ok(vec![0, 1, 2, 3, 4, 5]);
    }

    let mut indices = Vec::new();

    // Split by space or comma and filter out empty strings
    for segment in lower_input.split(|c: char| c == ' ' || c == ',').filter(|s| !s.is_empty()) {
        match segment.parse::<u8>() {
            Ok(die_num) if die_num >= 1 && die_num <= 6 => {
                // Convert 1-based index (1-6) to 0-based index (0-5)
                indices.push((die_num - 1) as usize);
            }
            _ => {
                // Return an error for invalid input segments
                return Err(format!("Invalid input segment: '{}'. Please use 1-6, 0, or X.", segment));
            }
        }
    }

    // Remove duplicates and sort for consistency
    indices.sort();
    indices.dedup();

    Ok(indices)
}

// --- 2. Dice Roll Implementation ---

/// Performs a full roll of all 6 dice and returns the results.
fn roll_dice() -> [DieResult; 6] {
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
fn reroll_dice(results: &mut [DieResult; 6], indices_to_reroll: &[usize]) {
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

/// Asks the player which dice to reroll (up to two times after the initial roll).
/// Returns the final set of dice results.
fn handle_reroll(initial_results: [DieResult; 6], max_rerolls: u8) -> [DieResult; 6] {
    let mut current_results = initial_results;
    let mut reroll_count = 0;

    while reroll_count < max_rerolls {
        // Display current dice state to the player (1-based index)
        println!("\n    🎲 Current Dice: ");
        for (i, result) in current_results.iter().enumerate() {
            print!("      [{}] = {:?} ", i + 1, result);
        }
        println!();

        let prompt = format!(
            "    ❓ Reroll {} of {}: Enter indices 1-6 (space/comma separated), 0 for all, or X to keep: ",
            reroll_count + 1,
            max_rerolls
        );
        let input = read_line_input(&prompt);

        match parse_reroll_input(&input) {
            Ok(indices_to_reroll) => {
                if indices_to_reroll.is_empty() {
                    // Player chose 'X' or entered nothing and wants to keep the dice
                    println!("    ✋ Keeping current dice results.");
                    break; // Exit the reroll loop
                }

                // Check if the input was '0' which results in a full reroll
                let full_reroll = input.trim() == "0";
                
                if indices_to_reroll.is_empty() && !full_reroll {
                    // This handles cases where the input was just empty or invalid (already filtered by parse_reroll_input)
                    // The logic here seems slightly redundant with the first check, but we keep it for robustness.
                    println!("    ⚠️ No dice selected. Moving to the next step.");
                    break;
                }

                // Perform the reroll
                reroll_dice(&mut current_results, &indices_to_reroll);
                reroll_count += 1;
            }
            Err(e) => {
                // Invalid input, prompt again without incrementing the reroll count
                println!("    ❌ Invalid input: {}", e);
                continue;
            }
        }
    }

    current_results
}

// --- 3. Game Logic Implementation ---

impl Game {
    fn new(player_names: &[&str]) -> Self {
        let players: Vec<Player> = player_names.iter()
            .enumerate()
            .map(|(i, &name)| Player::new(i as u32 + 1, name))
            .collect();

        Game {
            players,
            tokyo_controller_id: None,
            max_hp: 12,
            max_vp: 20,
        }
    }

    /// Finds a player by ID (used for getting mutable access).
    fn get_player_mut(&mut self, player_id: u32) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.id == player_id)
    }
    
    /// Finds a player by ID (used for getting read-only access).
    fn get_player(&self, player_id: u32) -> Option<&Player> {
        self.players.iter().find(|p| p.id == player_id)
    }

    /// Awards 2 VP for maintaining Tokyo control at the start of the turn.
    fn apply_tokyo_control_points(&mut self) {
        let max_vp = self.max_vp;

        if let Some(controller_id) = self.tokyo_controller_id {
            if let Some(player) = self.get_player_mut(controller_id) {
                player.victory_points = player.victory_points.saturating_add(2).min(max_vp);
                println!("    ⭐ **{}** MAINTAINS Tokyo control and gains +2 VP! (VP: {})", 
                         player.name, player.victory_points);
            }
        }
    }

    /// Processes all dice results for a player's turn, including user input for decisions.
    fn process_roll(&mut self, player_id: u32, results: &[DieResult; 6]) {
        let max_hp = self.max_hp;
        let max_vp = self.max_vp;

        let mut matched_numbers = 0;
        let player_is_in_tokyo = self.tokyo_controller_id == Some(player_id);

        println!("    Roll Results: {:?}", results);

        // Tally results
        let mut counts: HashMap<DieResult, i32> = HashMap::new(); 
        for &result in results {
            *counts.entry(result).or_insert(0) += 1;
        }

        // --- 1. Scoring: Matched Numbers (3 of a kind) ---
        // Scoring: The first set of 3 grants the VP. Any additional matching dice of the same face grant +1 VP each.
        if let Some(&count) = counts.get(&DieResult::One) {
            if count >= 3 { matched_numbers += 1 + (count - 3); }
        }
        if let Some(&count) = counts.get(&DieResult::Two) {
            if count >= 3 { matched_numbers += 2 + (count - 3); }
        }
        if let Some(&count) = counts.get(&DieResult::Three) {
            if count >= 3 { matched_numbers += 3 + (count - 3); }
        }
        // Note: The original code only awarded 1, 2, or 3 VP for *any* match of 3 or more.
        // The correction implements the standard rule of +1VP per extra die after 3.
        
        if matched_numbers > 0 {
            if let Some(player) = self.get_player_mut(player_id) {
                player.victory_points = player.victory_points.saturating_add(matched_numbers as u8).min(max_vp);
                println!("    ⭐ Matched numbers gain **{}** VP. (Total VP: {})", matched_numbers, player.victory_points);
            }
        }

        // --- 2. Energy, Hearts, and Claws ---
        let energy_count = counts.get(&DieResult::Energy).copied().unwrap_or(0);
        if energy_count > 0 {
            if let Some(player) = self.get_player_mut(player_id) {
                player.energy = player.energy.saturating_add(energy_count as u8);
                println!("    ⚡ Gains +{} Energy. (Total Energy: {})", energy_count, player.energy);
            }
        }

        let heart_count = counts.get(&DieResult::Heart).copied().unwrap_or(0);
        if heart_count > 0 {
            if !player_is_in_tokyo {
                if let Some(player) = self.get_player_mut(player_id) {
                    player.hp = player.hp.saturating_add(heart_count as u8).min(max_hp); 
                    println!("    ❤️ Gains +{} HP (Outside Tokyo). (Total HP: {})", heart_count, player.hp);
                }
            } else {
                 println!("    ❤️ Heart roll ignored: Player is in Tokyo.");
            }
        }
        
        let claw_count = counts.get(&DieResult::Claw).copied().unwrap_or(0);

        // --- 3. Attack and Tokyo Control ---
        if claw_count > 0 {
            if player_is_in_tokyo {
                // ATTACK: Damage to all OUTSIDE players
                println!("    💥 **ATTACK!** {} deals {} damage from Tokyo.", 
                         self.get_player(player_id).expect("Controller must exist").name, claw_count);

                let mut tokyo_conceded = false;

                for other_player in self.players.iter_mut().filter(|p| p.id != player_id) {
                    if self.tokyo_controller_id != Some(other_player.id) {
                        let damage = claw_count as u8;
                        other_player.hp = other_player.hp.saturating_sub(damage);
                        println!("        -> {} takes {} damage! (HP: {})", other_player.name, damage, other_player.hp);
                        
                        // Check if the attacked player is eliminated
                        if other_player.hp == 0 {
                            println!("        💀 {} has been eliminated!", other_player.name);
                            // If the eliminated player was the last outside player, it won't matter, 
                            // but the game loop will catch the victory condition later.
                        }
                    }
                }
                
                // DECISION: Concede Tokyo after attacking
                // Only allow concession if the player still has HP
                if let Some(controller) = self.get_player(player_id) {
                    if controller.hp > 0 {
                         let input = read_line_input(&format!("\n    ❓ {} has finished attacking. CONCEDE Tokyo? (y/N): ", controller.name));
                        
                         if input.eq_ignore_ascii_case("y") {
                            println!("    📢 {} CONCEDES Tokyo!", controller.name);
                            self.tokyo_controller_id = None;
                            tokyo_conceded = true;
                         }
                    }
                }

            } else {
                // CONTEST/ENTER TOKYO
                let current_controller = self.tokyo_controller_id;
                let player_name = self.get_player(player_id).expect("Player must exist").name.clone();

                let mut should_enter = false;

                if let Some(id) = current_controller {
                    // Tokyo is occupied. Challenger rolls claws.
                    let controller_name = self.get_player(id).expect("Controller must exist").name.clone();
                    
                    // Apply damage to the current controller
                    let damage = claw_count as u8;
                    if let Some(controller_player) = self.get_player_mut(id) {
                         controller_player.hp = controller_player.hp.saturating_sub(damage);
                         println!("    ⚔️  {} attacks {} in Tokyo for {} damage! (HP: {})", 
                                  player_name, controller_name, damage, controller_player.hp);
                        
                        if controller_player.hp == 0 {
                            println!("    💀 {} has been eliminated! Tokyo is vacant.", controller_name);
                            self.tokyo_controller_id = None;
                            should_enter = true;
                        }
                    }
                    
                    if self.tokyo_controller_id.is_some() {
                         // If the controller survived, they decide whether to concede
                         let input = read_line_input(&format!("\n    ❓ {} was attacked by {}. Should {} CONCEDE Tokyo? (y/N): ", 
                                                              controller_name, player_name, controller_name));
                        
                         if input.eq_ignore_ascii_case("y") {
                             println!("    📢 {} CONCEDES Tokyo!", controller_name);
                             self.tokyo_controller_id = None; // Tokyo is now vacant
                             should_enter = true;
                         } else {
                             println!("    🛡️ {} holds Tokyo against {}'s challenge.", controller_name, player_name);
                             return; // No change in control
                         }
                    } else {
                        // The controller was eliminated, so Tokyo is already vacant and `should_enter` is true.
                    }

                } else {
                    // Tokyo is vacant
                    should_enter = true;
                }

                if should_enter {
                    let input = read_line_input(&format!("    ❓ Tokyo is vacant. {} rolled {} Claw(s). Do you want to ENTER Tokyo? (Y/n): ", player_name, claw_count));

                    if !input.eq_ignore_ascii_case("n") {
                        self.tokyo_controller_id = Some(player_id);
                        if let Some(player) = self.get_player_mut(player_id) {
                            player.victory_points = player.victory_points.saturating_add(1).min(max_vp);
                            println!("    🚪 **{}** ENTERS Tokyo and gains +1 VP! (Total VP: {})", 
                                     player.name, player.victory_points);
                        }
                    } else {
                         println!("    🚫 {} declines to enter Tokyo.", player_name);
                    }
                }
            }
        }
    }

    /// Checks if the game has ended based on VP or HP conditions.
    fn check_victory_condition(&self) -> Option<String> {
        let active_players: Vec<&Player> = self.players.iter().filter(|p| p.hp > 0).collect();
        let max_vp = self.max_vp;

        // VP WIN
        if let Some(winner) = active_players.iter().find(|p| p.victory_points >= max_vp) {
            return Some(format!("{} reached {} Victory Points!", winner.name, max_vp));
        }

        // HP WIN (Last Kaiju Standing)
        if active_players.len() <= 1 {
            return if let Some(winner) = active_players.first() {
                Some(format!("{} is the Last Kaiju Standing!", winner.name))
            } else {
                // All players eliminated simultaneously
                Some(String::from("All Kaiju were eliminated simultaneously!"))
            };
        }

        None
    }
} 

// --- 4. Main Game Loop Implementation (Full Interactive Flow) ---

fn main() {
    println!("# 🦖 KING OF TOKYO (Simplified) 🏙️ #");
    
    // -----------------------------------------------------
    // Game Setup
    // -----------------------------------------------------
    let num_players_str = read_line_input("How many players (2-6)? ");
    let num_players: usize = num_players_str.parse().unwrap_or(2).min(6).max(2);
    
    let mut player_names = Vec::new();
    for i in 0..num_players {
        let name = read_line_input(&format!("Enter name for Player {}: ", i + 1));
        player_names.push(name);
    }
    
    let player_refs: Vec<&str> = player_names.iter().map(|s| s.as_str()).collect();
    let mut game = Game::new(&player_refs);
    
    // Initial Tokyo control based on starting player order (optional rule, simplifying here)
    if num_players >= 2 {
        game.tokyo_controller_id = Some(game.players[0].id);
        println!("*** {} starts in Tokyo and gains +1 VP! ***", game.players[0].name);
        game.players[0].victory_points = game.players[0].victory_points.saturating_add(1).min(game.max_vp);
    }

    println!("\n--- Game Start with {} Players ---", num_players);
    // -----------------------------------------------------
    
    let mut turn_count = 1;
    let mut current_player_index = 0;

    loop {
        // Ensure index is within bounds and cycles
        current_player_index %= game.players.len(); 

        let player_index = current_player_index;
        let current_player_id = game.players[player_index].id;
        let current_player_name = game.players[player_index].name.clone();
        
        // Skip dead players, but only if they have 0 HP
        if game.players[player_index].hp == 0 {
            current_player_index += 1;
            continue;
        }

        println!("\n---------------------------------------------------------");
        println!("--- Turn {} - {}'s Turn (HP: {}, VP: {}, Energy: {}) ---", 
                 turn_count, 
                 current_player_name, 
                 game.players[player_index].hp,
                 game.players[player_index].victory_points,
                 game.players[player_index].energy);
        
        let tokyo_status = if game.tokyo_controller_id == Some(current_player_id) {
            "You are in Tokyo."
        } else if game.tokyo_controller_id.is_some() {
            "Tokyo is occupied by another Kaiju."
        } else {
            "Tokyo is vacant."
        };
        println!("  Tokyo Status: {}", tokyo_status);
        println!("---------------------------------------------------------");
        
        // 1. Check for passive Tokyo VP
        game.apply_tokyo_control_points();

        // 2. Check for victory after Tokyo VP
        if let Some(message) = game.check_victory_condition() {
            println!("\n### 🎉 GAME OVER! ###");
            println!("{}", message);
            break;
        }

        // 3. Roll Dice
        let initial_roll = roll_dice();
        
        // 3.5 Handle Reroll (up to 2 rerolls after the initial roll, 3 total rolls)
        let final_results = handle_reroll(initial_roll, 2);

        // 4. Process Roll (Handles scoring, attack, and interactive Tokyo decisions)
        game.process_roll(current_player_id, &final_results);
        
        // 5. Check for victory after roll effects
        if let Some(message) = game.check_victory_condition() {
            println!("\n### 🎉 GAME OVER! ###");
            println!("{}", message);
            break;
        }

        // Move to next player
        current_player_index += 1;
        turn_count += 1;

        if turn_count > 1000 { 
            println!("\nGame stopped after 1000 turns for simulation limit.");
            break;
        }
    }
    
    // --- Final Tally ---
    println!("\n--- Final Scores ---");
    for player in game.players {
        println!("- {}: {} VP, {} HP, {} Energy", player.name, player.victory_points, player.hp, player.energy);
    }
}