use crate::algorithms::search::algorithms::heuristic_search::{SearchSettings, SearchSolution};
use crate::algorithms::search::lower_boundable::LowerBoundHeuristic;
use crate::algorithms::search::searchable::Searchable;

pub fn solve_problem<P, T>(problem: &P, search_settings: &SearchSettings<T>) -> SearchSolution<T>
    where
        P: Searchable + LowerBoundHeuristic
{
    SearchSolution::new(problem, search_settings, &|state| problem.lower_bound(state))
}