//state machine

use super::entity::Entity;
use super::entity_actions_queue::EntityCommand;
use super::entity_state::EntityState;
use super::entity_type::SurvivorData;

pub trait EntityStateMachine<T> {
    fn update(&self, entity: &mut Entity<T>);
}

pub struct SurvivorStateMachine {
    current_state: EntityState,
    previous_state: EntityState,
}

impl EntityStateMachine<SurvivorData> for SurvivorStateMachine {
    fn update(&self, entity: &mut Entity<SurvivorData>) {
        if entity.actions.is_empty() && entity.is_action_complete {
            entity.set_current_state(EntityState::Idle);
            return;
        }
        if !entity.is_action_complete {
            return;
        }

        let action = entity.get_next_action().unwrap();

        match action {
            EntityCommand::GoTo { destination } => {
                entity.set_current_state(EntityState::Moving);
            }
            EntityCommand::AcquireResource { destination } => {
                entity.set_current_state(EntityState::AcquiringResource);
            }
            EntityCommand::PickUpResource { destination } => {
                entity.set_current_state(EntityState::PickingUpResource);
            }
            EntityCommand::StoreResource { destination } => {
                entity.set_current_state(EntityState::StoringResource);
            }
            EntityCommand::Attack { target } => {
                entity.set_current_state(EntityState::Attacking);
            }
            EntityCommand::Build { destination } => {
                entity.set_current_state(EntityState::Building);
            }
            EntityCommand::Eat => {
                entity.set_current_state(EntityState::Eating);
            }
            EntityCommand::Drink => {
                entity.set_current_state(EntityState::Drinking);
            }
            EntityCommand::Sleep => {
                entity.set_current_state(EntityState::Sleeping);
            }
        }
    }
}
