use super::entity_actions_queue::EntityAction;
use super::entity_actions_queue::EntityActionsQueue;
use super::entity_state::EntityState;
use super::grid::GridPoint;

pub struct Entity {
    pub position: GridPoint,
    pub actions: EntityActionsQueue,
    pub state: EntityState,
}

impl Entity {
    pub fn new(position: GridPoint) -> Entity {
        Entity {
            position,
            actions: EntityActionsQueue::new(),
            state: EntityState::Idle,
        }
    }
    pub fn add_action(&mut self, action: EntityAction) {
        self.actions.push_back(action);
    }
}
