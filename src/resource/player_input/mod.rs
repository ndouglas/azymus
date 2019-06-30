use tcod::input::Event;

/// Resource.
#[derive(Clone, Copy, Debug)]
pub struct PlayerInputResource(pub Option<Event>);

impl Default for PlayerInputResource {
    fn default() -> Self {
        PlayerInputResource(None)
    }
}

#[cfg(test)]
mod tests {

}
