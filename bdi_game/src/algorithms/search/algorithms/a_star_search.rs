use crate::algorithms::search::algorithms::heuristic_search::{SearchSettings, SearchSolution};
use crate::algorithms::search::lower_boundable::LowerBoundable;
use crate::algorithms::search::searchable::Searchable;

pub fn solve_problem<'a, P>(problem: &mut P, search_settings: &SearchSettings<P>) -> SearchSolution<P>
    where
        P: Searchable + LowerBoundable + 'a
{
    SearchSolution::new(problem, search_settings, &|state| problem.lower_bound(state))
}