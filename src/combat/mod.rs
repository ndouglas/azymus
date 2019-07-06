use crate::effect;
use effect::Effect;
use crate::game;
use game::Game;

/// Attack.
pub mod attack;

/// Attack.
pub fn attack(attacker_id: usize, target_id: usize, game: &mut Game) {
    println!("Entering attack() with attacker {} and target {}.", attacker_id, target_id);
    let _context = attack::Context {
        attacker_id: attacker_id,
        defender_id: target_id,
    };
    Effect::DamageEntityBody(7).execute(target_id, game);
    println!("Exiting attack().");
}
