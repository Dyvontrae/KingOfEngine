// --- Core Data Structures + Library Core Game Function ---

use crate::player::Player;    // Use relative path within the crate
use crate::dice::DieResult;
use std::collections::HashMap;
use std::io::{self, Write}; // This imports the IO module and the Write trait



/// The central Game manager.
pub struct Game {
    pub players: Vec<Player>,
    pub tokyo_controller_id: Option<u32>, // ID of the player currently in Tokyo (or None)
    pub max_hp: u8,
    pub max_vp: u8,
}



// --- 3. Game Logic Implementation --- Package v.0.1; basic game logic

impl Game {
    pub fn new(player_names: &[&str]) -> Self {
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
    pub fn apply_tokyo_control_points(&mut self) {
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
    pub fn process_roll(&mut self, player_id: u32, results: &[DieResult; 6]) {
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
    pub fn check_victory_condition(&self) -> Option<String> {
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


// --- Helper Function for Reading Input ---

pub fn read_line_input(prompt: &str) -> String {
    print!("{}", prompt);
    // Flush the output buffer to ensure the prompt is displayed before input
    io::stdout().flush().expect("Failed to flush stdout"); 
    let mut input = String::new();
    // Use read_line to capture the input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}