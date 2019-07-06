use crate::component;
use component::position::Position;
use crate::game;
use game::Game;

/// A direct modification of the game world.
#[derive(Clone, Debug)]
pub enum Effect {
    /// Move the specified entity from one position to another.
    EntityMoves(Position, Position),
    /// Damage the entity by some amount.
    DamageEntityBody(i32),
    /// Kill the entity completely.
    KillEntity,
    /// Update the entity FOV.
    UpdateEntityFov,
}

/// A direct modification of the game world.
impl Effect {

    /// Perform the effect.
    pub fn execute(&self, id: usize, game: &mut Game) {
        use Effect::*;
        match self {
            EntityMoves(position1, position2) => {
                let mut entity = &mut game.entities[id];
                debug!("Moving entity {} from ({}, {}) to ({}, {}).", entity.name, position1.x, position1.y, position2.x, position2.y);
                entity.position = Some(*position2);
                UpdateEntityFov.execute(id, game);
            }
            DamageEntityBody(hp) => {
                println!("Entering DamageEntityBody() for id {}.", id);
                let entity = &mut game.entities[id];
                if let Some(body) = entity.body.as_mut() {
                    println!("Damaging entity {} ({} -> {}).", entity.name, body.current_hit_points, body.current_hit_points - hp);
                    body.current_hit_points -= hp;
                    if body.current_hit_points <= 0 {
                        KillEntity.execute(id, game);
                    }
                }
                println!("Exiting DamageEntityBody() for id {}.", id);
            }
            KillEntity => {
                println!("Entering KillEntity() for id {}.", id);
                let entity = &mut game.entities[id];
                println!("Killing entity {}!", entity.name);
                entity.nullify();
                println!("Entering KillEntity() for id {}.", id);
            }
            UpdateEntityFov => {
                let entity = &mut game.entities[id];
                if let Some(position) = &entity.position {
                    if let Some(fov) = &mut entity.field_of_view.as_mut() {
                        fov.update(position.x, position.y);
                    }
                }
            }
        }
    }

}
