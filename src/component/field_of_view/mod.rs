use tcod::map::FovAlgorithm;
use tcod::map::Map;
use std::sync::{Arc, Mutex};
use std::fmt;

/// Something representing an entity's ability to see the game world.
#[derive(Clone)]
pub struct FieldOfView {
    /// Algorithm.  See http://www.roguebasin.com/index.php?title=Comparative_study_of_field_of_view_algorithms_for_2D_grid_based_worlds
    pub algorithm: FovAlgorithm,
    /// Radius.
    pub radius: i32,
    /// The FOV map.
    pub map: Arc<Mutex<Map>>,
    /// The explored areas of the map.
    pub explored_map: Vec<Vec<bool>>,
    /// Last x-coordinate of viewer.
    pub x: i32,
    /// Last y-coordinate of viewer.
    pub y: i32,
    /// Full width of the map.
    pub width: i32,
    /// Full height of the map.
    pub height: i32,
}

/// Something representing an entity's ability to see the game world.
impl FieldOfView {

    /// Constructor.
    pub fn new(map: Map, radius: i32) -> FieldOfView {
        let (width, height) = map.size();
        let explored_map = vec![vec![false; height as usize]; width as usize];
        FieldOfView {
            algorithm: FovAlgorithm::Basic,
            radius: radius,
            map: Arc::new(Mutex::new(map)),
            explored_map: explored_map,
            x: -1,
            y: -1,
            width: width,
            height: height,
        }
    }

    /// Indicates whether a pair of coordinates are in bounds of this map.
    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        (x >= 0 && y >= 0 && x < self.width - 1 && y < self.height - 1)
    }

    /// Updates.
    pub fn update(&mut self, x: i32, y: i32) {
        let map = &mut self.map.lock().unwrap();
        map.compute_fov(x, y, self.radius, true, self.algorithm);
        self.x = x;
        self.y = y;
        for y2 in (y - self.radius)..(y + self.radius) {
            for x2 in (x - self.radius)..(x + self.radius) {
                if self.is_in_bounds(x2, y2) && !self.explored_map[x2 as usize][y2 as usize] && map.is_in_fov(x2, y2) {
                    self.explored_map[x2 as usize][y2 as usize] = true;
                }
            }
        }
    }

}

/// Allows us to show this object in tests, etc.
impl fmt::Debug for FieldOfView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldOfView")
    }
}
