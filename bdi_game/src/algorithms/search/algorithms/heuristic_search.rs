use crate::algorithms::search::searchable::{Searchable};
use crate::algorithms::search::algorithms::ongoing_search::{OngoingSearch, Path};
use crate::algorithms::search::lower_boundable::LowerBoundable;

pub struct SearchSettings<T: Searchable> {
    pub max_cost: T::Cost,
    pub use_max_cost: bool,
}

pub enum SearchSolution<P: Searchable> {
    Found(Path<P>),
    NotFound,
}

impl<P> SearchSolution<P> 
    where
        P: Searchable + LowerBoundable
{
    /*
    * If you don't want to use the heuristic function, you can use the NaiveSearchProblem struct
     */
    pub fn new(problem: &mut P, search_settings: &SearchSettings<P>) -> SearchSolution<P> {
        let mut ongoing_search = OngoingSearch::new(problem);

        while !ongoing_search.is_done() {
            if let Some(state) = ongoing_search.get_next_state() {
                if problem.is_goal(state) {
                    let path = ongoing_search.get_path(state);

                    return match path {
                        Some(path) => SearchSolution::Found(path),
                        None => SearchSolution::NotFound,
                    };
                }
                
                if search_settings.use_max_cost &&
                    ongoing_search.get_node_distance(state).map(|x| x.cost)
                    .unwrap_or(problem.get_zero_cost()) > search_settings.max_cost {
                    return SearchSolution::NotFound;
                }
                
                ongoing_search.relax_neighbours(state, &|state| problem.lower_bound(state));
            }
        }

        SearchSolution::NotFound
    }
}