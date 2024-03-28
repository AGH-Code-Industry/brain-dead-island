use crate::algorithms::search::algorithms::ongoing_search::{OngoingSearch, Path};
use crate::algorithms::search::lower_boundable::LowerBoundable;
use crate::algorithms::search::searchable::Searchable;

pub struct SearchSettings<'a, T: Searchable<'a>> {
    pub max_cost: T::Cost,
    pub use_max_cost: bool,
}

pub enum SearchSolution<'a, P: Searchable<'a>> {
    Found(Path<'a, P>),
    NotFound,
}

impl<'a, P> SearchSolution<'a, P>
where
    P: Searchable<'a> + LowerBoundable<'a>,
{
    /*
     * If you don't want to use the heuristic function, you can use the NaiveSearchProblem struct
     */
    pub fn new(
        problem: &'a mut P,
        search_settings: &SearchSettings<'a, P>,
    ) -> SearchSolution<'a, P> {
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

                if search_settings.use_max_cost
                    && ongoing_search
                        .get_node_distance(state)
                        .map(|x| x.cost)
                        .unwrap_or(problem.get_zero_cost())
                        > search_settings.max_cost
                {
                    return SearchSolution::NotFound;
                }

                ongoing_search.relax_neighbours(state, &|state| problem.lower_bound(state));
            }
        }

        SearchSolution::NotFound
    }
}
