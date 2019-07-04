use crate::component;
use component::position::Position;
use super::MapType;
use crate::entity;
use entity::Entity;

/// Algorithms.
pub mod algorithm;

/// The type returned from map generators.
pub type MapGeneratorReturnType = (MapType, Position);

/// The type of function of a map generator.
pub type MapGeneratorType = fn(i64, i32, i32, i32, &mut Vec<Entity>) -> MapGeneratorReturnType;
