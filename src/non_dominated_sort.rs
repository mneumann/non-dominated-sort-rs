use domination::DominationOrd;
use std::cmp::Ordering;
use std::collections::VecDeque;

pub struct SolutionWithIndex {
    /// The index that `solution` has within the `solutions` array.
    pub index: usize,
}

pub struct Front {
    /// The first front has rank 0, second rank 1 and so on.
    pub rank: usize,

    // The solutions within this front
    pub solutions: Vec<SolutionWithIndex>,
}

impl Front {
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

    rank: isize,

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
    current_rank: isize,
}

impl<'a, S> NonDominatedSort<'a, S> {
    /// Perform a non-dominated sort of `solutions`.
    ///
    /// Each pareto front (the indices of the `solutions`) can be obtained by calling `next()`.

    pub fn new<D>(solutions: &'a [S], domination: &D) -> Self
    where
        D: DominationOrd<Solution = S>,
    {
        let mut entries: Vec<_> = solutions
            .iter()
            .enumerate()
            .map(|(index, solution)| Entry {
                solution,
                index,
                domination_count: 0,
                dominated_solutions: VecDeque::new(),
                rank: -1,
            })
            .collect();

        let current_rank = 0;

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
                    p.rank = current_rank;
                }
            }
        }

        Self {
            entries,
            current_rank,
        }
    }

    /// Returns an array containing all pareto fronts.
    pub fn pareto_fronts(self) -> Vec<Front> {
        self.into_iter().collect()
    }

    /// Returns an array containing the first pareto fronts, until
    /// `max_solutions` have been found. Note that always the whole fronts are
    /// returned, i.e. the number of solutions returned may be higher.
    pub fn pareto_fronts_stop_at(self, max_solutions: usize) -> Vec<Front> {
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
    type Item = Front;

    /// Return the next pareto front

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate the next front based on the current front, which
        // might be empty, in which case the next_front will be empty as

        let mut next_front = Front {
            rank: self.current_rank as usize,
            solutions: Vec::new(),
        };

        for e_i in 0..self.entries.len() {
            if self.entries[e_i].rank == self.current_rank {
                debug_assert!(self.entries[e_i].domination_count == 0);
                // This entry belongs to the current front, as it is
                // not-dominated by any other solution
                next_front.solutions.push(SolutionWithIndex {
                    index: self.entries[e_i].index,
                });

                //
                while let Some(q_i) = self.entries[e_i].dominated_solutions.pop_front() {
                    let q = &mut self.entries[q_i];
                    debug_assert!(q.domination_count > 0);
                    q.domination_count -= 1;
                    if q.domination_count == 0 {
                        // q is not dominated by any other solution. it belongs to the next front.
                        // next_front.solutions.push(SolutionWithIndex { index: q_i });
                        // mark for next round
                        q.rank = self.current_rank + 1;
                    }
                }
            }
        }

        self.current_rank += 1;

        if next_front.solutions.is_empty() {
            None
        } else {
            Some(next_front)
        }
    }
}
