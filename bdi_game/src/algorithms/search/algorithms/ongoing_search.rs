use std::collections::{HashMap, HashSet};
use priority_queue::PriorityQueue;
use crate::algorithms::search::searchable::Searchable;

struct NodeDistance<'a, 'b, T: Searchable> {
    pub previous: Option<&'a T>,
    pub cost: &'b T::Cost,
}

pub struct Path<T> {
    pub states: Vec<T>,
    pub cost: u32,
}

pub struct OngoingSearch<'a, 'b, T: Searchable> {
    problem: T,
    distances: HashMap<T, NodeDistance<'a, 'b, T>>,
    priority_queue: PriorityQueue<T, &'b T::Cost>,
    was_visited: HashSet<T>,
}

impl<T> OngoingSearch<T>
    where
        T: Searchable
{
    pub fn new(problem: T) -> OngoingSearch<T> {
        let mut distances = HashMap::new();
        let mut priority_queue = PriorityQueue::new();
        let mut was_visited = HashSet::new();

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

    pub fn get_node_distance(&self, state: &T) -> Option<&NodeDistance<T>> {
        self.distances.get(state)
    }

    pub fn relax_neighbours(&mut self, state: T,
                            heuristic_function: &impl Fn(&T) -> T::Cost) {
        let neighbours = self.problem.get_neighbors(&state);
        let current_cost_info = self.get_current_total_cost(&state)
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
                previous: Some(&state),
                cost: new_cost,
            };

            if let Some(current_neighbour_cost) = self.get_current_total_cost(&neighbour) {
                if new_cost < *current_neighbour_cost {
                    self.distances.insert(&neighbour, new_cost_info);
                }
            } else {
                self.distances.insert(&neighbour, new_cost_info);
            }
            
            let priority = new_cost + heuristic_function(&neighbour);
            self.priority_queue.push(&neighbour, &priority);
        }
    }

    pub fn is_done(&self) -> bool {
        self.priority_queue.is_empty()
    }

    pub fn get_next_state(&mut self) -> Option<T> {
        loop {
            let next_state = self.priority_queue.pop();

            return if let Some(ref state) = next_state {
                if self.was_visited.contains(state) {
                    continue;
                }

                self.was_visited.insert(state.clone());
                next_state
            } else {
                None
            }
        }
    }

    pub fn was_state_visited(&self, state: &T) -> bool {
        self.was_visited.contains(state)
    }

    pub fn get_path(&self, state: T) -> Option<Path<T>> {
        let mut path = vec![];
        let mut current_state = state;
        let mut cost = 0;

        if !self.was_state_visited(&current_state) {
            return None;
        }

        while let Some(distance_info) = self.get_node_distance(&current_state) {
            path.push(current_state.clone());
            cost += distance_info.cost;
            if let Some(previous_state) = distance_info.previous {
                current_state = previous_state.clone();
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
