use crate::algorithms::search::searchable::{Searchable};
use priority_queue::PriorityQueue;
use crate::algorithms::search::algorithms::ongoing_search::{OngoingSearch, Path};
use crate::algorithms::search::lower_boundable::LowerBoundHeuristic;

pub struct SearchSettings<T> {
    pub max_cost: T::Cost,
}

pub enum SearchSolution<T> {
    Found(Path<T>),
    NotFound,
}

impl<T> SearchSolution<T> {
    pub fn new<P>(problem: &P, search_settings: &SearchSettings<T>,
                  heuristic_function: &impl Fn(&T) -> T::Cost) -> SearchSolution<T>
        where
            P: Searchable
    {
        let mut ongoing_search = OngoingSearch::new(problem);

        while !ongoing_search.is_done() {
            if let Some(state) = ongoing_search.pop_next_state() {
                if problem.is_goal(&state) {
                    let path = ongoing_search.get_path(&state);
                    
                    return match path {
                        Some(path) => SearchSolution::Found(path),
                        None => SearchSolution::NotFound,
                    };
                }
                
                if ongoing_search.get_node_distance(&state).map(|x| x.cost)
                    .unwrap_or(problem.get_zero_cost()) > search_settings.max_cost {
                    return SearchSolution::NotFound;
                }
                
                ongoing_search.relax_neighbours(state, &heuristic_function);
            }
        }
        
        SearchSolution::NotFound
    }
}