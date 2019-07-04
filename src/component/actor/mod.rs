/// Something that gets dispensed time and has an opportunity to act.
#[derive(Clone, Copy, Debug)]
pub struct Actor {
    /// The current time of the entity.
    pub time: i32,
    /// The time acquired by the entity each turn.
    pub speed: i32,
}
