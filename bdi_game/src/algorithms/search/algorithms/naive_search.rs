use crate::algorithms::search::algorithms::heuristic_search::{SearchSettings, SearchSolution};
use crate::algorithms::search::searchable::Searchable;

pub fn solve_problem<P, T>(problem: &P, search_settings: &SearchSettings<T>) -> SearchSolution<T>
    where
        P: Searchable
{
    SearchSolution::new(problem, search_settings, &|_| problem.get_zero_cost())
}
