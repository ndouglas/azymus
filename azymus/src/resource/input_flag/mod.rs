/// Resource.
#[derive(Clone, Copy, Debug)]
pub struct InputFlagResource(pub bool);

impl Default for InputFlagResource {
    fn default() -> Self {
        InputFlagResource(false)
    }
}

#[cfg(test)]
mod tests {

}
