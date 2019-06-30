use std::fmt;

/// Resource.
pub struct OpaqueMapResource(pub Vec<Vec<bool>>);

/// Allows us to show this object in tests, etc.
impl fmt::Debug for OpaqueMapResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpaqueMapResource")
    }
}

#[cfg(test)]
mod tests {

}
