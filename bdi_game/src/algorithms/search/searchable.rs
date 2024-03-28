use std::hash::Hash;
use std::ops::{Add, AddAssign};

pub struct StatesArc<T, C: Ord> {
    pub target: T,
    pub cost: C,
}

pub trait Searchable<'a> {
    type State: Hash + Eq + Clone;
    type Cost: Ord + Copy + Default + Add<Output=Self::Cost> + AddAssign;

    fn get_start(&self) -> &'a Self::State;
    fn get_goal(&self) -> &'a Self::State;
    fn get_neighbors(&self, state: &'a Self::State) -> Vec<StatesArc<&'a Self::State, Self::Cost>>;
    fn get_zero_cost(&self) -> Self::Cost;
    fn is_goal(&self, state: &'a Self::State) -> bool;
}
