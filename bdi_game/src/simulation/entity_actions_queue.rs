use std::collections::VecDeque;
use super::grid::GridPoint;
pub enum EntityCommand {
    GoTo{destination: GridPoint},
    AcquireResource{destination: GridPoint},//chop wood, mine stone, mine iron, mine gold, acquire food 
    PickUpResource{destination: GridPoint},
    StoreResource{destination: GridPoint},
    Attack{target: GridPoint},
    Build{destination: GridPoint},
    Eat,
    Drink,
    Sleep,
}


pub type EntityActionsQueue = VecDeque<EntityCommand>;
