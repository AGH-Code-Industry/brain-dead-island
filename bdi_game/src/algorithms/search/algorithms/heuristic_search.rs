use crate::algorithms::search::searchable::{Searchable, OngoingSearch};
use priority_queue::PriorityQueue;
use crate::algorithms::search::lower_boundable::LowerBoundHeuristic;

pub struct SearchSettings {
    pub max_depth: u32,
    pub max_cost: u32,
}

pub enum SearchSolution<T> {
    Found(Path<T>),
    NotFound,
}

impl<T> SearchSolution<T> {
    pub fn new<P>(problem: P) -> SearchSolution<T>
        where
            P: Searchable + LowerBoundHeuristic 
    {        
        let mut ongoing_search = OngoingSearch::new(problem);

        while !ongoing_search.is_done() {
            if let Some(state) = ongoing_search.pop_next_state() {
                
            }
        }
        
        SearchSolution::NotFound
    }
}