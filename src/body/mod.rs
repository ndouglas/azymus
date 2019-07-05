/// A body is the physical representation of an actor entity.
#[derive(Clone, Copy, Debug)]
pub struct Body {
    /// The total hit points that this body has.
    pub total_hit_points: i32,
    /// The current hit points that this body has.
    pub current_hit_points: i32,
}
