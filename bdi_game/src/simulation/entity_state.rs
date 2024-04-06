#[derive(PartialEq, Copy, Clone, Debug)]
pub enum EntityState {
    Idle,
    Moving,
    AcquiringResource,
    PickingUpResource,
    StoringResource,
    Attacking,
    Building,
    Eating,
    Drinking,
    Sleeping,
    RunningAway,
}

pub type EntityStateHistory = Vec<EntityState>;