use crate::algorithms::search::searchable::Searchable;

pub trait LowerBoundable<'a>: Searchable<'a> {
    fn lower_bound(&self, state: &Self::State) -> Self::Cost;
}
