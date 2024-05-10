use super::grid::GridPoint;
use std::collections::VecDeque;
pub enum EntityAction {
    GoTo { destination: GridPoint },
}

pub type EntityActionsQueue = VecDeque<EntityAction>;
