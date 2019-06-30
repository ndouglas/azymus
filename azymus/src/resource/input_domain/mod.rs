use crate::input;
use input::Domain;

/// Resource.
#[derive(Clone, Copy, Debug)]
pub struct InputDomainResource(pub Domain);

impl Default for InputDomainResource {
    fn default() -> Self {
        InputDomainResource(Domain::Explore)
    }
}

#[cfg(test)]
mod tests {

}
