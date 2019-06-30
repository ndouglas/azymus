use specs::{Component, VecStorage};
use tcod::map::FovAlgorithm;
use tcod::map::Map;
use std::sync::{Arc, Mutex};
use std::fmt;

/// The position of an object in the world.
#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct FieldOfView {

    /// Algorithm.  See http://www.roguebasin.com/index.php?title=Comparative_study_of_field_of_view_algorithms_for_2D_grid_based_worlds
    pub algorithm: FovAlgorithm,

    /// Radius.
    pub radius: i32,

    /// The FOV map.
    pub map: Option<Arc<Mutex<Map>>>,

    /// Last x-coordinate of viewer.
    pub x: i32,

    /// Last y-coordinate of viewer.
    pub y: i32,
}

/// Allows us to show this object in tests, etc.
impl fmt::Debug for FieldOfView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldOfView")
    }
}
