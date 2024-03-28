use crate::algorithms::search::algorithms::heuristic_search::{SearchSettings, SearchSolution};
use crate::algorithms::search::lower_boundable::LowerBoundable;
use crate::algorithms::search::searchable::Searchable;

pub fn solve_problem<'a, P>(
    problem: &'a mut P,
    search_settings: &SearchSettings<'a, P>,
) -> SearchSolution<'a, P>
where
    P: Searchable<'a> + LowerBoundable<'a> + 'a,
{
    SearchSolution::new(problem, search_settings)
}
