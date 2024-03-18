use crate::algorithms::search::algorithms::heuristic_search::{SearchSettings, SearchSolution};
use crate::algorithms::search::searchable::Searchable;

pub fn solve_problem<'a, P>(problem: &'a mut P, search_settings: &SearchSettings<P>) -> SearchSolution<P>
    where
        P: Searchable + 'a
{
    let zero_cost = problem.get_zero_cost();
    
    SearchSolution::new(problem, search_settings, &|_| zero_cost)
}
