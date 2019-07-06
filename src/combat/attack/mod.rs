
/// An attack context.
#[derive(Clone, Copy, Debug)]
pub struct Context {
    /// The entity ID of the attacker.
    pub attacker_id: usize,
    /// The entity ID of the defender.
    pub defender_id: usize,
}
