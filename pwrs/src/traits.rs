use king_core::player::Player; // Use the actual player from core

pub trait HenshinPower {
    fn on_energy_spent(&self, player: &mut Player, amount: u8);
    fn calculate_bonus_damage(&self, roll_sum: u32) -> u32;
}