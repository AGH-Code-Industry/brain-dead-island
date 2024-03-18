use crate::algorithms::search::searchable::Searchable;

pub trait LowerBoundable: Searchable {
    fn lower_bound(&self, state: &Self::State) -> Self::Cost;
}
