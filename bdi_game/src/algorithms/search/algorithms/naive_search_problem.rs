use crate::algorithms::search::lower_boundable::LowerBoundable;
use crate::algorithms::search::searchable::{Searchable, StatesArc};

struct NaiveSearchProblem<'a, P: Searchable<'a>> {
    problem: &'a P,
}

impl<'a, P> Searchable<'a> for NaiveSearchProblem<'a, P>
where
    P: 'a + Searchable<'a>,
{
    type State = P::State;
    type Cost = P::Cost;

    fn get_start(&self) -> &'a Self::State {
        self.problem.get_start()
    }

    fn get_goal(&self) -> &'a Self::State {
        self.problem.get_goal()
    }

    fn get_neighbors(&self, state: &'a Self::State) -> Vec<StatesArc<&'a Self::State, Self::Cost>> {
        self.problem.get_neighbors(state)
    }

    fn get_zero_cost(&self) -> Self::Cost {
        self.problem.get_zero_cost()
    }

    fn is_goal(&self, state: &'a Self::State) -> bool {
        self.problem.is_goal(state)
    }
}

impl<'a, P> LowerBoundable<'a> for NaiveSearchProblem<'a, P>
where
    P: Searchable<'a> + 'a,
{
    fn lower_bound(&self, _: &Self::State) -> Self::Cost {
        self.problem.get_zero_cost()
    }
}

impl<'a, P> NaiveSearchProblem<'a, P>
where
    P: Searchable<'a> + 'a,
{
    pub fn new(problem: &'a P) -> NaiveSearchProblem<'a, P> {
        NaiveSearchProblem { problem }
    }
}
