use crate::component;
use component::position::Position;
use crate::game;
use game::Game;
//use crate::map;
//use map::Map;
use crate::math;
use math::geometry::cell::Cell;
use math::geometry::cell::Cellular;
use crate::species;
use species::Factory as SpeciesFactory;

/// MoveEntity.
pub mod move_entity;

/// A direct modification of the game world.
#[derive(Clone, Debug)]
pub enum Effect {
    /// Move the specified entity from one position to another.
    MoveEntity(Cell, Cell),
    /// Damage the entity by some amount.
    DamageEntityBody(usize, i32),
    /// Remove the entity entirely.
    RemoveEntity,
    /// Kill the entity completely.
    KillEntity,
    /// Update the entity FOV.
    UpdateEntityFov,
    /// Create a new entity at this position.
    CreateEntity(Position, SpeciesFactory),
    /// Change species of the specified entity.
    ChangeEntitySpecies(SpeciesFactory),
}

/// A direct modification of the game world.
impl Effect {

    /// Perform the effect.
    pub fn execute(&self, id: usize, game: &mut Game) {
        use Effect::*;
        match self {
            MoveEntity(cell1, cell2) => {
                let move_entity = move_entity::MoveEntity::new(id, *cell1, *cell2);
                move_entity.execute(&mut game.map);
                UpdateEntityFov.execute(id, game);
            },
            DamageEntityBody(target_id, hp) => {
                let entity = &mut game.entities[*target_id];
                trace!("Entering DamageEntityBody() for {}.", entity);
                if let Some(body) = entity.body.as_mut() {
                    // debug!("Damaging {} ({} -> {}).", entity, body.current_hit_points, body.current_hit_points - hp);
                    body.current_hit_points -= hp;
                    if body.current_hit_points <= 0 {
                        debug!("{} reduced below 0 hit points.", entity);
                        KillEntity.execute(*target_id, game);
                    }
                }
                trace!("Exiting DamageEntityBody() for id {}.", target_id);
            },
            KillEntity => {
                let entity = &mut game.entities[id];
                trace!("Killing entity {}", entity);
                entity.species = None;
                entity.body = None;
                entity.actor = None;
                entity.agent = None;
                entity.field_of_view = None;
                entity.light_source = None;
                if let Some(renderable) = entity.renderable.as_mut() {
                    //debug!("Transforming {} into corpse.", entity);
                    renderable.char = Some('%');
                    renderable.background_color = None;
                }
                entity.blocks_movement = false;
                trace!("Exiting KillEntity() for id {}.", id);
            },
            RemoveEntity => {
                println!("Entering RemoveEntity() for id {}.", id);
                let entity = &mut game.entities[id];
                entity.species = None;
                entity.body = None;
                entity.actor = None;
                entity.agent = None;
                entity.field_of_view = None;
                entity.light_source = None;
                entity.renderable = None;
                if let Some(position) = &entity.position {
                    game.map.entity_map.remove_entity_id(id, &position.as_cell());
                }
            },
            UpdateEntityFov => {
                println!("Entering UpdateEntityFov() for id {}.", id);
                let entity = &mut game.entities[id];
                if let Some(position) = &entity.position {
                        println!("Updating FoV for {:?}.", entity);
                    if let Some(fov) = &mut entity.field_of_view.as_mut() {
                        fov.update(position.x, position.y);
                    }
                }
            },
            CreateEntity(position, species_factory) => {
                println!("Entering CreateEntity({:?}, {:?}) for id {}.", position, species_factory, id);
                let mut entity = species_factory.create();
                entity.position = Some(*position);
                let id = game.entities.len();
                entity.id = id;
                game.entities.push(entity);
                game.map.entity_map.insert_entity_id(id, &position.as_cell());
            },
            ChangeEntitySpecies(species_factory) => {
                println!("Entering ChangeEntitySpecies({:?}) for id {}.", species_factory, id);
                let new_entity = species_factory.create();
                let old_entity = &mut game.entities[id];
                old_entity.set(&new_entity);
            },
        }
    }

}
