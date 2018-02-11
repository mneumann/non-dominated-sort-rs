use domination::DominationOrd;
use std::cmp::Ordering;
use std::mem;
use std::collections::VecDeque;

pub struct SolutionWithIndex<'a, S>
where
    S: 'a,
{
    /// A reference to the solution
    pub solution: &'a S,

    /// The index that `solution` has within the `solutions` array.
    pub index: usize,
}

pub struct Front<'a, S>
where
    S: 'a,
{
    /// The first front has rank 0, second rank 1 and so on.
    pub rank: usize,

    // The solutions within this front
    pub solutions: Vec<SolutionWithIndex<'a, S>>,
}

impl<'a, S> Front<'a, S>
where
    S: 'a,
{
    pub fn solutions_indices_only(&self) -> Vec<usize> {
        self.solutions.iter().map(|s| s.index).collect()
    }
}

struct Entry<'a, S>
where
    S: 'a,
{
    /// A reference to the solution
    solution: &'a S,

    /// The index that `solution` has within the `solutions` array.
    index: usize,

    /// By how many other solutions is this solution dominated
    domination_count: usize,

    /// Which solutions we dominate
    dominated_solutions: VecDeque<usize>,
}

pub struct NonDominatedSort<'a, S>
where
    S: 'a,
{
    entries: Vec<Entry<'a, S>>,
    current_front: Front<'a, S>,
}

impl<'a, S> NonDominatedSort<'a, S> {
    /// Perform a non-dominated sort of `solutions`.
    ///
    /// Each pareto front (the indices of the `solutions`) can be obtained by calling `next()`.

    pub fn new<D>(solutions: &'a [S], domination: &D) -> Self
    where
        D: DominationOrd<Solution = S>,
    {
        let mut current_front = Front {
            rank: 0,
            solutions: Vec::new(),
        };

        let mut entries: Vec<_> = solutions
            .iter()
            .enumerate()
            .map(|(index, solution)| Entry {
                solution,
                index,
                domination_count: 0,
                dominated_solutions: VecDeque::new(),
            })
            .collect();

        for start in 0..entries.len() {
            let mut iter = entries[start..].iter_mut();
            if let Some(p) = iter.next() {
                for q in iter {
                    match domination.domination_ord(p.solution, q.solution) {
                        Ordering::Less => {
                            // p dominates q
                            // Add `q` to the set of solutions dominated by `p`.
                            p.dominated_solutions.push_back(q.index);
                            // q is dominated by p
                            q.domination_count += 1;
                        }
                        Ordering::Greater => {
                            // p is dominated by q
                            // Add `p` to the set of solutions dominated by `q`.
                            q.dominated_solutions.push_back(p.index);
                            // q dominates p
                            // Increment domination counter of `p`.
                            p.domination_count += 1;
                        }
                        Ordering::Equal => {}
                    }
                }
                if p.domination_count == 0 {
                    // `p` belongs to the first front as it is not dominated by any
                    // other solution.
                    current_front.solutions.push(SolutionWithIndex {
                        solution: p.solution,
                        index: p.index,
                    });
                }
            }
        }

        Self {
            entries,
            current_front,
        }
    }

    /// Returns an array containing all pareto fronts.
    pub fn pareto_fronts(self) -> Vec<Front<'a, S>> {
        self.into_iter().collect()
    }

    /// Returns an array containing the first pareto fronts, until
    /// `max_solutions` have been found. Note that always the whole fronts are
    /// returned, i.e. the number of solutions returned may be higher.
    pub fn pareto_fronts_stop_at(self, max_solutions: usize) -> Vec<Front<'a, S>> {
        let mut found_solutions = 0;
        let mut fronts = Vec::new();

        for front in self {
            found_solutions += front.solutions.len();
            fronts.push(front);
            if found_solutions >= max_solutions {
                break;
            }
        }
        return fronts;
    }
}

/// Iterate over the pareto fronts. Each call to next() will yield the
/// next pareto front.
impl<'a, S> Iterator for NonDominatedSort<'a, S> {
    type Item = Front<'a, S>;

    /// Return the next pareto front

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate the next front based on the current front, which
        // might be empty, in which case the next_front will be empty as
        // well and we stop.

        let mut next_front = Front {
            rank: self.current_front.rank + 1,
            solutions: Vec::new(),
        };

        for p in self.current_front.solutions.iter() {
            // to calculate the next front, we have to remove the
            // solutions of the current front, and as such, decrease the
            // domination_count of they dominated_solutions. We can
            // destruct the dominated_solutions array here, as we will
            // no longer need it.
            // The only problem with poping off solutions off the end is
            // that we will populate the fronts in reverse order. For
            // that reason, we are using a VecDeque. This should give us
            // a stable sort.

            while let Some(q_i) = self.entries[p.index].dominated_solutions.pop_front() {
                let q = &mut self.entries[q_i];
                debug_assert!(q.domination_count > 0);
                q.domination_count -= 1;
                if q.domination_count == 0 {
                    // q is not dominated by any other solution. it belongs to the next front.
                    next_front.solutions.push(SolutionWithIndex {
                        solution: q.solution,
                        index: q_i,
                    });
                }
            }
        }

        // swap current with next front
        let current_front = mem::replace(&mut self.current_front, next_front);

        if current_front.solutions.is_empty() {
            None
        } else {
            Some(current_front)
        }
    }
}
