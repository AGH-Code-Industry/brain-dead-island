use crate::algorithms::search::searchable::{Searchable};
use crate::algorithms::search::algorithms::ongoing_search::{OngoingSearch, Path};

pub struct SearchSettings<T: Searchable> {
    pub max_cost: T::Cost,
}

pub enum SearchSolution<P: Searchable> {
    Found(Path<P>),
    NotFound,
}

impl<P> SearchSolution<P> 
    where
        P: Searchable
{
    pub fn new(problem: &mut P, search_settings: &SearchSettings<P>,
                  heuristic_function: &impl Fn(&P::State) -> P::Cost) -> SearchSolution<P> {
        let mut ongoing_search = OngoingSearch::new(problem);

        while !ongoing_search.is_done() {
            if let Some(state) = ongoing_search.get_next_state() {
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