/// Resource.
#[derive(Clone, Copy, Debug)]
pub struct ContinueFlagResource(pub bool);

impl Default for ContinueFlagResource {
    fn default() -> Self {
        ContinueFlagResource(true)
    }
}

#[cfg(test)]
mod tests {

}
