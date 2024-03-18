use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use priority_queue::PriorityQueue;

pub struct StatesArc<T, C: Ord> {
    pub target: T,
    pub cost: C,
}

pub trait Searchable {
    type State: Hash + Eq;
    type Cost: Ord;

    fn get_start(&self) -> Self::State;
    fn get_goal(&self) -> Self::State;
    fn get_neighbors(&self, state: &Self::State) -> Vec<StatesArc<Self::State, Self::Cost>>;
    fn get_zero_cost(&self) -> Self::Cost;
    fn is_goal(&self, state: &Self::State) -> bool;
}
