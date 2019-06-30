use std::fmt;

/// Resource.
pub struct OccupantMapResource(pub Vec<Vec<bool>>);

/// Allows us to show this object in tests, etc.
impl fmt::Debug for OccupantMapResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OccupantMap")
    }
}

#[cfg(test)]
mod tests {

}
