use domination::DominationOrd;
use std::cmp::Ordering;

pub trait Index: Copy {
    fn check(max_index: usize) -> bool;
    fn new(index: usize) -> Self;
    fn index(self) -> usize;
    fn inc(&mut self);
    fn dec_to_zero(&mut self) -> bool;
}

impl Index for usize {
    fn new(index: usize) -> Self {
        index
    }

    fn check(_max_index: usize) -> bool {
        true
    }

    fn index(self) -> usize {
        self
    }

    fn inc(&mut self) {
        *self += 1;
    }
    fn dec_to_zero(&mut self) -> bool {
        *self -= 1;
        *self == 0
    }
}

impl Index for u32 {
    fn check(max_index: usize) -> bool {
        max_index <= u32::max_value() as usize
    }

    fn new(index: usize) -> Self {
        index as u32
    }

    fn index(self) -> usize {
        self as usize
    }

    fn inc(&mut self) {
        *self += 1;
    }
    fn dec_to_zero(&mut self) -> bool {
        *self -= 1;
        *self == 0
    }
}

pub struct Front<'a, S, I = usize>
where
    S: 'a,
    I: Index,
{
    dominated_solutions: Vec<Vec<I>>,
    domination_count: Vec<I>,
    current_front: Vec<I>,
    rank: usize,
    solutions: &'a [S],
}

impl<'a, S, I> Front<'a, S, I>
where
    S: 'a,
    I: Index,
{
    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn is_empty(&self) -> bool {
        self.current_front.is_empty()
    }

    pub fn current_front_indices(&self) -> &[I] {
        &self.current_front[..]
    }

    pub fn next_front(self) -> Self {
        let Front {
            dominated_solutions,
            mut domination_count,
            current_front,
            rank,
            solutions,
        } = self;

        let mut next_front = Vec::new();

        for p_i in current_front.into_iter() {
            for &q_i in dominated_solutions[p_i.index()].iter() {
                debug_assert!(domination_count[q_i.index()].index() > 0);
                if domination_count[q_i.index()].dec_to_zero() {
                    // q_i is not dominated by any other solution. it belongs to the next front.
                    next_front.push(q_i);
                }
            }
        }

        Self {
            dominated_solutions,
            domination_count,
            current_front: next_front,
            rank: rank + 1,
            solutions,
        }
    }
}

/// Perform a non-dominated sort of `solutions`. Returns the first
/// pareto front.
pub fn non_dominated_sort<'a, S, D, I>(solutions: &'a [S], domination: &D) -> Front<'a, S, I>
where
    D: DominationOrd<Solution = S>,
    S: 'a,
    I: Index,
{
    assert!(I::check(solutions.len()));

    // The indices of the solutions that are dominated by this `solution`.
    let mut dominated_solutions: Vec<Vec<I>> = solutions.iter().map(|_| Vec::new()).collect();

    // For each solutions, we keep a domination count, i.e.
    // the number of solutions that dominate the solution.
    // If this count is negative, it is the rank of the front.
    let mut domination_count: Vec<I> = solutions.iter().map(|_| I::new(0)).collect();

    let mut current_front: Vec<I> = Vec::new();

    // inital pass over each combination: O(n*n / 2).
    let mut iter = solutions.iter().enumerate();
    while let Some((p_i, p)) = iter.next() {
        let mut pair_iter = iter.clone();
        while let Some((q_i, q)) = pair_iter.next() {
            match domination.domination_ord(p, q) {
                Ordering::Less => {
                    // p dominates q
                    // Add `q` to the set of solutions dominated by `p`.
                    dominated_solutions[p_i.index()].push(I::new(q_i));
                    // q is dominated by p
                    domination_count[q_i.index()].inc();
                }
                Ordering::Greater => {
                    // p is dominated by q
                    // Add `p` to the set of solutions dominated by `q`.
                    dominated_solutions[q_i.index()].push(I::new(p_i));
                    // q dominates p
                    // Increment domination counter of `p`.
                    domination_count[p_i.index()].inc();
                }
                Ordering::Equal => {}
            }
        }
        // if domination_count drops to zero, push index to front.
        if domination_count[p_i.index()].index() == 0 {
            current_front.push(I::new(p_i));
        }
    }

    Front {
        dominated_solutions,
        domination_count,
        current_front,
        rank: 0,
        solutions,
    }
}
