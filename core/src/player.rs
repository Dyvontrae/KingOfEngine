/// Represents a single Kaiju player's state.
#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub hp: u8,         // Max 12, start 10
    pub victory_points: u8, // Max 20
    pub energy: u8,     // Currency
}


impl Player {
    pub fn new(id: u32, name: &str) -> Self {
        Player {
            id,
            name: name.to_string(),
            hp: 10, // Start HP
            victory_points: 0,
            energy: 0,
        }
    }
}