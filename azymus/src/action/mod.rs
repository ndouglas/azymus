use specs::*;
use crate::component;
use component::position::Position;
use crate::resource;
use resource::continue_flag::ContinueFlagResource;
use resource::root_console::RootConsoleResource;
use crate::rule::ActionRule;

/// The actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    /// Quits the game.
    QuitGame,
    /// Toggles fullscreen.
    ToggleFullscreen,
    /// Walk in some direction.
    Walk((i32, i32),(i32,i32)),
}

impl Action {

    /// Execute the action.
    pub fn execute(self, entity: Entity, world: &mut World) {
        use Action::*;
        match self {
            QuitGame => {
                trace!("Quitting game!");
                let continue_flag_resource = &mut world.write_resource::<ContinueFlagResource>().0;
                *continue_flag_resource = false;
            },
            ToggleFullscreen => {
                let root_console_resource = &mut world.write_resource::<RootConsoleResource>().0;
                let root_console = &mut root_console_resource.lock().unwrap();
                let fullscreen = root_console.is_fullscreen();
                root_console.set_fullscreen(!fullscreen);
            },
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
        Action::ToggleFullscreen                            =>  { [].to_vec() }
        Action::QuitGame                                    =>  { [].to_vec() },
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
