use super::grid::GridPoint;
use std::collections::VecDeque;
pub enum EntityCommand {
    GoTo { destination: GridPoint },
}

pub type EntityActionsQueue = VecDeque<EntityCommand>;
