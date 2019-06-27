use specs::*;
use crate::component;
use component::position::Position;
use crate::rule::ActionRule;

const DEFAULT_ACTION_COST: i32 = 100;
//const NO_ACTION_COST: i32 = 0;

/// The actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    /// Walk in some direction.
    Walk((i32, i32),(i32,i32)),
}

impl Action {

    /// Execute the action.
    pub fn execute(self, entity: Entity, world: &mut World) {
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

}

/// Get action rules.
///
/// Returns the rules that may transform or cancel the attempted action.
pub fn get_action_rules(action: Action) -> Vec<ActionRule> {
    use ActionRule::*;
    match action {
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
pub fn get_permitted_action(action: Action, entity: Entity, world: &World) -> Option<Action> {
    let rules = get_action_rules(action);
    if rules.len() == 0 {
        return Some(action);
    }
    for rule in rules {
        match rule.callback(action, entity, world) {
            Some(action2) => {
                if action == action2 {
                    continue;
                }
                return get_permitted_action(action2, entity, world);
            },
            None => {
                return None;
            },
        }
    }
    return Some(action);
}

/// Get action cost.
///
/// Returns the cost of an action.
pub fn get_action_cost(action: Action, _entity: Entity, _world: &World) -> i32 {
    use Action::*;
    match action {
        Walk((_, _), (_, _))                                => DEFAULT_ACTION_COST,
    }
}
