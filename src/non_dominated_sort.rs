use domination::DominationOrd;
use std::cmp::Ordering;
use std::mem;
use std::collections::VecDeque;

pub struct SolutionWithIndex<'a, S, I: Copy = usize>
where
    S: 'a,
{
    /// A reference to the solution
    solution: &'a S,

    /// The index that `solution` has within the `solutions` array.
    index: I,
}

pub struct Front<'a, S, I = usize>
where
    S: 'a,
    I: Copy,
{
    /// The first front has rank 0, second rank 1 and so on.
    rank: usize,

    // The solutions within this front
    solutions: Vec<SolutionWithIndex<'a, S, I>>,
}

impl<'a, S, I> Front<'a, S, I>
where
    S: 'a,
    I: Copy,
{
    pub fn solutions_indices_only(&self) -> Vec<I> {
        self.solutions.iter().map(|s| s.index).collect()
    }
}

struct Entry<'a, S, I = usize>
where
    S: 'a,
{
    /// A reference to the solution
    solution: &'a S,

    /// The index that `solution` has within the `solutions` array.
    index: I,

    /// By how many other solutions is this solution dominated
    domination_count: I,

    /// Which solutions we dominate
    dominated_solutions: VecDeque<I>,
}

pub struct NonDominatedSort<'a, S, I: Copy = usize>
where
    S: 'a,
{
    entries: Vec<Entry<'a, S, I>>,
    current_front: Front<'a, S, I>,
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

        for mid in 1..entries.len() + 1 {
            let (front_slice, tail_slice) = entries.split_at_mut(mid);
            debug_assert!(front_slice.len() > 0);
            let p = front_slice.last_mut().unwrap();
            for q in tail_slice.iter_mut() {
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

        Self {
            entries,
            current_front,
        }
    }

    /// Returns an array containing all pareto fronts.
    pub fn pareto_fronts(self) -> Vec<Front<'a, S, usize>> {
        self.into_iter().collect()
    }

    /// Returns an array containing the first pareto fronts, until
    /// `max_solutions` have been found. Note that always the whole fronts are
    /// returned, i.e. the number of solutions returned may be higher.
    pub fn pareto_fronts_stop_at(self, max_solutions: usize) -> Vec<Front<'a, S, usize>> {
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
    type Item = Front<'a, S, usize>;

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

#[cfg(test)]
mod helper {
    use test_helper_domination::Tuple;
    use super::Front;
    pub fn get_solutions() -> Vec<Tuple> {
        vec![
            Tuple(1, 2),
            Tuple(1, 2),
            Tuple(2, 1),
            Tuple(1, 3),
            Tuple(0, 2),
        ]
    }

    pub fn assert_front_eq(
        expected_rank: usize,
        expected_indices: &[usize],
        front: &Front<Tuple, usize>,
    ) {
        assert_eq!(expected_rank, front.rank);
        assert_eq!(expected_indices.to_owned(), front.solutions_indices_only());
    }
}

#[test]
fn test_non_dominated_sort() {
    use test_helper_domination::TupleDominationOrd;
    let solutions = helper::get_solutions();
    let fronts = NonDominatedSort::new(&solutions, &TupleDominationOrd).pareto_fronts();

    assert_eq!(3, fronts.len());
    helper::assert_front_eq(0, &[2, 4], &fronts[0]);
    helper::assert_front_eq(1, &[0, 1], &fronts[1]);
    helper::assert_front_eq(2, &[3], &fronts[2]);
}

#[test]
fn test_non_dominated_sort_stop_at() {
    use test_helper_domination::TupleDominationOrd;
    let solutions = helper::get_solutions();

    {
        let fronts =
            NonDominatedSort::new(&solutions, &TupleDominationOrd).pareto_fronts_stop_at(2);
        assert_eq!(1, fronts.len());
        helper::assert_front_eq(0, &[2, 4], &fronts[0]);
    }
    {
        let fronts =
            NonDominatedSort::new(&solutions, &TupleDominationOrd).pareto_fronts_stop_at(3);
        assert_eq!(2, fronts.len());
        helper::assert_front_eq(0, &[2, 4], &fronts[0]);
        helper::assert_front_eq(1, &[0, 1], &fronts[1]);
    }
}

#[test]
fn test_non_dominated_sort_iter() {
    use test_helper_domination::TupleDominationOrd;
    let solutions = helper::get_solutions();
    let mut fronts = NonDominatedSort::new(&solutions, &TupleDominationOrd);

    {
        let f0 = fronts.next();
        assert!(f0.is_some());
        helper::assert_front_eq(0, &[2, 4], &f0.unwrap());
    }
    {
        let f1 = fronts.next();
        assert!(f1.is_some());
        helper::assert_front_eq(1, &[0, 1], &f1.unwrap());
    }
    {
        let f2 = fronts.next();
        assert!(f2.is_some());
        helper::assert_front_eq(2, &[3], &f2.unwrap());
    }
    {
        let f3 = fronts.next();
        assert!(f3.is_none());
    }
}
