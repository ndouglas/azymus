use crate::species;
use species::Species;

/// Faction class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Faction {
    /// The player faction.
    Player,
    /// A species-based faction.
    Species(Species),
}

/// Penalties given by the faction when laws are broken.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Penalty {
    /// The member is a permanent outcast of the faction and is attacked on sight.
    Pariah,
    /// The member's existence is completely ignored; they are neither acknowledged nor discussed.
    Unperson,
    /// The member is exiled from the faction for a number of turns.
    Exile(i32),
    /// The member is imprisoned for a number of turns.
    Jail(i32),
}

/// Penalties/Rewards given by the faction when laws are broken.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Consequence {
    /// Standing with the faction changes by X.
    ChangeStanding(i32),
    /// Some penalty is imposed.
    Penalty(Penalty),
    /// The member is beaten nearly to death.
    Assault,
    /// The member is killed.
    Death,
    /// The member is abandoned.
    Abandoned,
    /// The member is released from prison early.
    Unjail,
    /// The member is released from exile early.
    Unexile,
}

/// Laws governing the behavior of this faction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Laws {
    /// Must always maintain a minimum standing.
    MinimumStanding(i32, Consequence),
    /// Must not attack a member of a given faction.
    DoNotAttackFaction(Faction, Consequence),
    /// Must not assist a member of a given faction.
    DoNotAssistFaction(Faction, Consequence),
}

/// Faction membership.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Standing {
    /// The faction.
    pub faction: Faction,
    /// The entity's standing with the faction.
    pub standing: i32,
    /// Any penalties the entity might be under with the faction.
    pub penalty: Option<Penalty>,
}
