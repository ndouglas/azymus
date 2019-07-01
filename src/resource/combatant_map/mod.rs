use std::fmt;

/// Resource.
pub struct CombatantMapResource(pub Vec<Vec<bool>>);

/// Allows us to show this object in tests, etc.
impl fmt::Debug for CombatantMapResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CombatantMap")
    }
}

#[cfg(test)]
mod tests {

}
