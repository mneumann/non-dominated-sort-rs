use domination::DominationOrd;
use std::cmp::Ordering;

pub struct Front<'a, S: 'a> {
    dominated_solutions: Vec<Vec<usize>>,
    domination_count: Vec<usize>,
    previous_front: Vec<usize>,
    current_front: Vec<usize>,
    rank: usize,
    solutions: &'a [S],
}

impl<'a, S: 'a> Front<'a, S> {
    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn is_empty(&self) -> bool {
        self.current_front.is_empty()
    }

    pub fn current_front_indices(&self) -> &[usize] {
        &self.current_front[..]
    }

    pub fn next_front(self) -> Self {
        let Front {
            dominated_solutions,
            mut domination_count,
            previous_front,
            current_front,
            rank,
            solutions,
        } = self;

        // reuse the previous_front
        let mut next_front = previous_front;
        next_front.clear();

        for &p_i in current_front.iter() {
            for &q_i in dominated_solutions[p_i].iter() {
                debug_assert!(domination_count[q_i] > 0);
                domination_count[q_i] -= 1;
                if domination_count[q_i] == 0 {
                    // q_i is not dominated by any other solution. it belongs to the next front.
                    next_front.push(q_i);
                }
            }
        }

        Self {
            dominated_solutions,
            domination_count,
            previous_front: current_front,
            current_front: next_front,
            rank: rank + 1,
            solutions,
        }
    }
}

/// Perform a non-dominated sort of `solutions`. Returns the first
/// pareto front.
pub fn non_dominated_sort<'a, S, D>(solutions: &'a [S], domination: &D) -> Front<'a, S>
where
    D: DominationOrd<Solution = S>,
{
    // The indices of the solutions that are dominated by this `solution`.
    let mut dominated_solutions: Vec<Vec<usize>> = solutions.iter().map(|_| Vec::new()).collect();

    // For each solutions, we keep a domination count, i.e.
    // the number of solutions that dominate the solution.
    // If this count is negative, it is the rank of the front.
    let mut domination_count: Vec<usize> = solutions.iter().map(|_| 0).collect();

    let mut current_front: Vec<usize> = Vec::new();

    // inital pass over each combination: O(n*n / 2).
    let mut iter = solutions.iter().enumerate();
    while let Some((p_i, p)) = iter.next() {
        let mut pair_iter = iter.clone();
        while let Some((q_i, q)) = pair_iter.next() {
            match domination.domination_ord(p, q) {
                Ordering::Less => {
                    // p dominates q
                    // Add `q` to the set of solutions dominated by `p`.
                    dominated_solutions[p_i].push(q_i);
                    // q is dominated by p
                    domination_count[q_i] += 1;
                }
                Ordering::Greater => {
                    // p is dominated by q
                    // Add `p` to the set of solutions dominated by `q`.
                    dominated_solutions[q_i].push(p_i);
                    // q dominates p
                    // Increment domination counter of `p`.
                    domination_count[p_i] += 1
                }
                Ordering::Equal => {}
            }
        }
        // if domination_count drops to zero, push index to front.
        if domination_count[p_i] == 0 {
            current_front.push(p_i);
        }
    }

    Front {
        dominated_solutions,
        domination_count,
        previous_front: Vec::new(),
        current_front,
        rank: 0,
        solutions,
    }
}
