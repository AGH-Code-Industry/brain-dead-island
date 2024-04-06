use super::grid::GridPoint;
use super::entity_type::SurvivorData;
use super::entity_type::ZombieData;
use super::entity_state::EntityState;
use super::entity_state::EntityStateHistory;
use super::entity_actions_queue::EntityCommand;
use super::entity_actions_queue::EntityActionsQueue;
pub type Survivor = Entity<SurvivorData>;
pub type Zombie = Entity<ZombieData>;

pub struct Entity<EntityType> //survivor or zombie
{
    pub id: u64,
    pub position: GridPoint,
    pub data: EntityType,
    pub actions: EntityActionsQueue,
    pub state: EntityState,
    pub state_history: EntityStateHistory,
    pub is_action_complete: bool,
}

impl<EntityType> Entity<EntityType> {
    pub fn new(id: u64, position: GridPoint, data: EntityType) -> Entity<EntityType> {
        Entity {
            id,
            position,
            data,
            actions: EntityActionsQueue::new(),
            state: EntityState::Idle,
            state_history: EntityStateHistory::new(),
            is_action_complete: false,
        }
    }
    pub fn add_action(&mut self, action: EntityCommand) {
        self.actions.push_back(action);
    }

    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }

    pub fn get_next_action(&mut self) -> Option<EntityCommand> {
        self.actions.pop_front()
    }

    pub fn return_action(&mut self, action: EntityCommand) {
        self.actions.push_front(action);
    }

    pub fn get_current_state(&self) -> EntityState {
        self.state
    }

    pub fn set_current_state(&mut self, state: EntityState) {
        self.state = state;
    }

    
}