/// Resource.
#[derive(Clone, Copy, Debug)]
pub struct TurnFlagResource(pub bool);

impl Default for TurnFlagResource {
    fn default() -> Self {
        TurnFlagResource(true)
    }
}

#[cfg(test)]
mod tests {

}
