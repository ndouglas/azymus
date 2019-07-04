/// Default time consumed per user turn.
const DEFAULT_TIME_PER_TURN: i32 = 120;
/// Default delta time per tick.
const DEFAULT_TIME_PER_TICK: i32 = 1;

/// Scheduler-specific settings.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Scheduler {
    /// Time consumed per user turn.
    pub time_per_turn: i32,
    /// Delta time per tick.
    pub time_per_tick: i32,
}

/// Scheduler-specific settings.
impl Scheduler {

    /// Constructor.
    pub fn new() -> Self {
        Scheduler {
            time_per_turn: DEFAULT_TIME_PER_TURN,
            time_per_tick: DEFAULT_TIME_PER_TICK,
        }
    }

}

impl Default for Scheduler {
    fn default() -> Self {
        Scheduler::new()
    }
}

#[cfg(test)]
mod tests {

}
