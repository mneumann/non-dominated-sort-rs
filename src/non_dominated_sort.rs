use domination::DominationOrd;
use std::cmp::Ordering;

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

    /// Which solutions we dominate
    dominated_solutions: Vec<usize>,
}

struct DominationInfo {
    /// By how many other solutions is this solution dominated.
    /// If negative (or 0), this describes the front this solution has
    /// been assigned.
    domination_count: isize,
}

pub struct NonDominatedSort<'a, S>
where
    S: 'a,
{
    entries: Vec<Entry<'a, S>>,
    current_rank: isize,
    max_rank: isize,
    dominations: Vec<DominationInfo>,
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
                dominated_solutions: Vec::new(),
            })
            .collect();

        let mut dominations: Vec<_> = solutions
            .iter()
            .map(|_| DominationInfo {
                domination_count: 0,
            })
            .collect();

        // inital pass
        for start in 0..entries.len() {
            let mut iter = entries[start..].iter_mut();
            if let Some(p) = iter.next() {
                for q in iter {
                    match domination.domination_ord(p.solution, q.solution) {
                        Ordering::Less => {
                            // p dominates q
                            // Add `q` to the set of solutions dominated by `p`.
                            p.dominated_solutions.push(q.index);
                            // q is dominated by p
                            dominations[q.index].domination_count += 1;
                        }
                        Ordering::Greater => {
                            // p is dominated by q
                            // Add `p` to the set of solutions dominated by `q`.
                            q.dominated_solutions.push(p.index);
                            // q dominates p
                            // Increment domination counter of `p`.
                            dominations[p.index].domination_count += 1;
                        }
                        Ordering::Equal => {}
                    }
                }
            }
        }

        let mut rank = 0;
        loop {
            if entries.is_empty() {
                break;
            }
            let mut e_i = 0;
            loop {
                if e_i >= entries.len() {
                    break;
                }

                let idx = entries[e_i].index;
                if dominations[idx].domination_count == rank {
                    // This entry belongs to the current front, as it is
                    // not-dominated by any other solution
                    let entry = entries.swap_remove(e_i);

                    for q_i in entry.dominated_solutions.into_iter() {
                        let q = &mut dominations[q_i];
                        debug_assert!(q.domination_count > 0);
                        q.domination_count -= 1;
                        if q.domination_count == 0 {
                            // q is not dominated by any other solution. it belongs to the next front.
                            // next_front.solutions.push(SolutionWithIndex { index: q_i });
                            // mark for next round
                            q.domination_count = rank - 1;
                        }
                    }
                // DO not increase index e_i here, as we swapped in
                // an element from the back.
                // continue;
                } else {
                    e_i += 1;
                }
            }

            rank -= 1;
        }

        Self {
            entries,
            current_rank: 0,
            max_rank: rank,
            dominations,
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
        let next_front = Front {
            rank: -self.current_rank as usize,
            solutions: self.dominations
                .iter()
                .enumerate()
                .filter(|&(_i, d)| d.domination_count == self.current_rank)
                .map(|(i, _d)| SolutionWithIndex { index: i })
                .collect(),
        };

        if next_front.solutions.is_empty() {
            return None;
        } else {
            self.current_rank -= 1;
            return Some(next_front);
        }
    }
}
