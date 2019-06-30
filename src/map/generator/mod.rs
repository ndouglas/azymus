use super::MapType;
use super::MapCoordinateType;

/// Algorithms.
pub mod algorithm;

/// The type returned from map generators.
pub type MapGeneratorReturnType = (MapType, MapCoordinateType);
