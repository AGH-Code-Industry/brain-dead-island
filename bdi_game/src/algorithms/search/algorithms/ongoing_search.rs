use std::collections::{HashMap, HashSet};
use priority_queue::PriorityQueue;
use crate::algorithms::search::searchable::Searchable;

pub struct NodeDistance<'a, P: Searchable> {
    pub previous: Option<&'a P::State>,
    pub cost: P::Cost,
}

pub struct Path<P: Searchable> {
    pub states: Vec<P::State>,
    pub cost: P::Cost,
}

pub struct OngoingSearch<'a, P: Searchable> {
    problem: &'a P,
    distances: HashMap<&'a P::State, NodeDistance<'a, P>>,
    priority_queue: PriorityQueue<&'a P::State, P::Cost>,
    was_visited: HashSet<&'a P::State>,
}

impl<'a, P: Searchable> OngoingSearch<'a, P>
    where
        P: Searchable + 'a
{
    pub fn new(problem: &'a P) -> OngoingSearch<'a, P> {
        let mut distances = HashMap::new();
        let mut priority_queue = PriorityQueue::new();
        let was_visited = HashSet::new();

        distances.insert(problem.get_start(),
                         NodeDistance {
                             previous: None,
                             cost: problem.get_zero_cost(),
                         }
        );

        priority_queue.push(problem.get_start(), problem.get_zero_cost());

        OngoingSearch {
            problem,
            distances,
            priority_queue,
            was_visited,
        }
    }

    pub fn get_node_distance(&self, state: &P::State) -> Option<&NodeDistance<P>> {
        self.distances.get(state)
    }

    pub fn relax_neighbours(&mut self, state: &'a P::State,
                            heuristic_function: &impl Fn(&P::State) -> P::Cost) {
        let neighbours = self.problem.get_neighbors(&state);
        let current_cost_info = self.get_node_distance(&state)
            .expect("State not found in distances");

        let current_cost = current_cost_info.cost;

        for neighbour_arc in neighbours {
            let neighbour = neighbour_arc.target;

            if self.was_visited.contains(&neighbour) {
                continue;
            }

            let neighbour_cost = neighbour_arc.cost;
            let new_cost = current_cost + neighbour_cost;

            let new_cost_info = NodeDistance {
                previous: Some(state),
                cost: new_cost,
            };

            if let Some(current_neighbour_cost) = self.get_node_distance(&neighbour) {
                if new_cost < current_neighbour_cost.cost {
                    self.distances.insert(&neighbour, new_cost_info);
                }
            } else {
                self.distances.insert(&neighbour, new_cost_info);
            }
            
            let priority = new_cost + heuristic_function(&neighbour);
            self.priority_queue.push(&neighbour, priority);
        }
    }

    pub fn is_done(&self) -> bool {
        self.priority_queue.is_empty()
    }

    pub fn get_next_state(&mut self) -> Option<&'a P::State> {
        loop {
            let next_state_maybe = self.priority_queue.pop();

            return if let Some((&ref state, _)) = next_state_maybe {
                if self.was_visited.contains(&state) {
                    continue;
                }

                self.was_visited.insert(&state);
                Some(state)
            } else {
                None
            }
        }
    }

    pub fn was_state_visited(&self, state: &P::State) -> bool {
        self.was_visited.contains(state)
    }

    pub fn get_path(&self, state: &'a P::State) -> Option<Path<P>> {
        let mut path = vec![];
        let mut current_state = state;
        let mut cost = self.problem.get_zero_cost();

        if !self.was_state_visited(&current_state) {
            return None;
        }

        while let Some(&ref distance_info) = self.get_node_distance(&current_state) {
            path.push(current_state.clone());
            cost += distance_info.cost;
            if let Some(previous_state) = &distance_info.previous {
                current_state = previous_state;
            } else {
                break;
            }
        }

        path.reverse();

        Some(Path {
            states: path,
            cost,
        })
    }
}
