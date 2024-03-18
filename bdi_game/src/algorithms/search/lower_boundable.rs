pub trait LowerBoundHeuristic {
    type State;
    type Cost: Ord;

    fn lower_bound(&self, state: &Self::State) -> Self::Cost;
}