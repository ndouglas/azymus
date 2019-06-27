use specs::*;
use crate::component;
use component::actor::Actor;
use component::position::Position;
use crate::rule::ActionRule;

const DEFAULT_ACTION_COST: i32 = 100;

/// The actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    /// Walk in some direction.
    Walk((i32, i32),(i32,i32)),
}

impl Action {

    /// Execute the action.
    fn inner_execute(self, entity: Entity, world: &mut World) {
        use Action::*;
        match self {
            Walk((_x1, _y1), (x2, y2)) => {
                let position_storage = &mut (world.write_storage::<Position>());
                if let Some(position) = &mut position_storage.get_mut(entity) {
                    position.x = x2;
                    position.y = y2;
                }
            },
        };
    }

    /// Execute the action.
    pub fn execute(self, entity: Entity, world: &mut World) {
        if let Some(action) = self.get_permitted_action(entity, world) {
            let cost = action.get_cost(entity, world);
            action.inner_execute(entity, world);
            let actor_storage = &mut (world.write_storage::<Actor>());
            if let Some(actor) = actor_storage.get_mut(entity) {
                trace!("Deducting {} energy from actor ({} -> {}).", cost, actor.energy, actor.energy - cost);
                actor.energy -= cost;
            }
        }
    }

    /// Get action rules.
    ///
    /// Returns the rules that may transform or cancel the attempted action.
    pub fn get_rules(self) -> Vec<ActionRule> {
        use ActionRule::*;
        match self {
            Action::Walk((_x1, _y1), (x2, y2))                  =>  {
                                                                        [
                                                                            CheckMapBoundaries(x2, y2),
                                                                            CheckMapOccupied(x2, y2),
                                                                        ].to_vec()
                                                                    },
        }
    }

    /// Get action rules.
    ///
    /// Returns the rules that may transform or cancel the attempted action.
    pub fn get_permitted_action(self, entity: Entity, world: &World) -> Option<Action> {
        let rules = self.get_rules();
        if rules.len() == 0 {
            return Some(self);
        }
        for rule in rules {
            match rule.callback(self, entity, world) {
                Some(action) => {
                    if self == action {
                        continue;
                    }
                    return action.get_permitted_action(entity, world);
                },
                None => {
                    return None;
                },
            }
        }
        return Some(self);
    }

    /// Get action cost.
    ///
    /// Returns the cost of an action.
    pub fn get_cost(self, _entity: Entity, _world: &World) -> i32 {
        use Action::*;
        match self {
            Walk((_, _), (_, _))                                => DEFAULT_ACTION_COST,
        }
    }

}
