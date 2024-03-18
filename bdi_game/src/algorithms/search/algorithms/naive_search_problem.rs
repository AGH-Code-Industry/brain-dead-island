use crate::algorithms::search::lower_boundable::LowerBoundable;
use crate::algorithms::search::searchable::{Searchable, StatesArc};

struct NaiveSearchProblem<'a, P: Searchable> {
    problem: &'a P,
}

impl<'a, P> Searchable for NaiveSearchProblem<'a, P> where P: 'a + Searchable {
    type State = P::State;
    type Cost = P::Cost;

    fn get_start(&self) -> &Self::State {
        self.problem.get_start()
    }

    fn get_goal(&self) -> &Self::State {
        self.problem.get_goal()
    }

    fn get_neighbors(&self, state: &Self::State) -> Vec<StatesArc<&Self::State, Self::Cost>> {
        self.problem.get_neighbors(state)
    }

    fn get_zero_cost(&self) -> Self::Cost {
        self.problem.get_zero_cost()
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        self.problem.is_goal(state)
    }
}

impl<'a, P> LowerBoundable for NaiveSearchProblem<'a, P>
    where
        P: Searchable + 'a{
    fn lower_bound(&self, _: &Self::State) -> Self::Cost {
        self.problem.get_zero_cost()
    }
}

impl<'a, P> NaiveSearchProblem<'a, P>
    where
        P: Searchable + 'a
{
    pub fn new(problem: &'a P) -> NaiveSearchProblem<'a, P> {
        NaiveSearchProblem {
            problem,
        }
    }
}
