#[derive(Debug)]
pub struct DenseState<'a> {
    shape: &'a Vec<usize>,
    indices: Vec<usize>,
    compound: usize,
}

pub struct DenseOneBodyIterator<'a> {
    start_state: &'a DenseState<'a>,
    next_state: DenseState<'a>,
    pos: usize,
    num_iterations: usize,
}

pub struct DenseTwoBodyIterator<'a> {
    start_state: &'a DenseState<'a>,
    next_state: DenseState<'a>,
    one_body_iterator: DenseOneBodyIterator<'a>,
    pos_r: usize,
    pos_l: usize,
    num_iterations: usize,
}

impl<'a> DenseState<'a> {
    fn new_init_state(shape: &'a Vec<usize>) -> Self {
        let indices = vec![0; shape.len()];

        Self {
            shape,
            indices,
            compound: 0,
        }
    }

    fn new_from_indices(indices: Vec<usize>, shape: &'a Vec<usize>) -> Self {
        let mut compound = 0;
        let mut prod = 1;

        for i in (0..indices.len()).rev() {
            compound += indices[i] * prod;
            prod *= shape[i];
        }

        Self {
            shape,
            indices,
            compound,
        }
    }

    fn new_from_compound(compound: usize, shape: &'a Vec<usize>) -> Self {
        let mut indices = vec![0; shape.len()];
        let mut prod = 1;

        for i in (0..shape.len()).rev() {
            indices[i] = (compound / prod) % shape[i];
            prod *= shape[i];
        }

        Self {
            shape,
            indices,
            compound,
        }
    }
}

impl<'a> DenseOneBodyIterator<'a> {
    fn new(start_state: &'a DenseState<'a>) -> Self {
        DenseOneBodyIterator::new_with_pos(
            start_state,
            start_state.shape.len() - 1,
        )
    }

    fn new_with_pos(start_state: &'a DenseState<'a>, pos: usize) -> Self {
        let mut next_inds = start_state.indices.clone();
        let mut pos = pos;

        for i in (0..start_state.shape.len()).rev() {
            if start_state.shape[i] > 1 {
                next_inds[i] = (next_inds[i] + 1) % start_state.shape[i];
                pos = i;
                break;
            }
        }

        Self {
            start_state,
            next_state: DenseState::new_from_indices(
                next_inds,
                start_state.shape,
            ),
            pos,
            num_iterations: 0,
        }
    }
}

impl<'a> DenseTwoBodyIterator<'a> {
    fn new(start_state: &'a DenseState<'a>) -> Self {
        let mut next_inds = start_state.indices.clone();
        let mut pos_r = start_state.shape.len() - 1;
        let mut pos_l = match pos_r.checked_sub(1) {
            Some(c) => c,
            None => {
                panic!("The start_state needs to contain at least two indices")
            }
        };

        for i in (0..pos_r).rev() {
            if start_state.shape[i] > 1 {
                next_inds[i] = (next_inds[i] + 1) % start_state.shape[i];
                pos_l = i;
                break;
            }
        }

        let one_body_iterator =
            DenseOneBodyIterator::new_with_pos(start_state, pos_l);

        Self {
            start_state,
            next_state: DenseState::new_from_indices(
                next_inds,
                start_state.shape,
            ),
            one_body_iterator,
            pos_r,
            pos_l,
            num_iterations: 0,
        }
    }
}

impl<'a> Iterator for DenseOneBodyIterator<'a> {
    type Item = (i64, DenseState<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_iterations > 0
            && self.start_state.indices == self.next_state.indices
        {
            // We have done some iterations, and come back to the start state,
            // hence we must be done.
            return None;
        }

        let mut new_inds = self.next_state.indices.clone();

        loop {
            new_inds[self.pos] =
                (new_inds[self.pos] + 1) % self.start_state.shape[self.pos];

            if new_inds[self.pos] == self.start_state.indices[self.pos] {
                // We have iterated through all values at this position.
                // Decrement self.pos, and try next position.
                if self.pos > 0 {
                    self.pos -= 1;
                    continue;
                }
            }

            self.num_iterations += 1;
            return Some((
                1,
                std::mem::replace(
                    &mut self.next_state,
                    DenseState::new_from_indices(
                        new_inds,
                        self.start_state.shape,
                    ),
                ),
            ));
        }
    }
}

impl<'a> Iterator for DenseTwoBodyIterator<'a> {
    type Item = (i64, DenseState<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: YOU ARE HERE!
        if self.num_iterations > 0
            && self.start_state.indices == self.next_state.indices
        {
            return None;
        }

        let mut new_inds = self.next_state.indices.clone();

        Some((1, DenseState::new_init_state(self.start_state.shape)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_init_state_dense() {
        let shape = vec![3, 4, 5];
        let init_state = DenseState::new_init_state(&shape);

        for x in init_state.indices {
            assert!(x == 0);
        }

        assert!(init_state.compound == 0);
    }

    #[test]
    fn test_new_from_indices_and_compound_dense() {
        let shape = vec![3, 4, 5];
        let mut counter = 0;

        for p in 0..shape[0] {
            for q in 0..shape[1] {
                for r in 0..shape[2] {
                    let state =
                        DenseState::new_from_indices(vec![p, q, r], &shape);
                    let state_2 =
                        DenseState::new_from_compound(counter, &shape);

                    assert!(state.compound == counter);
                    assert!(state.compound == state_2.compound);

                    for (x, y) in state
                        .indices
                        .into_iter()
                        .zip(state_2.indices.into_iter())
                    {
                        assert!(x == y);
                    }

                    counter += 1;
                }
            }
        }
    }

    #[test]
    fn test_dense_iterator_one_manual() {
        let shape = vec![3, 4, 1];
        let state = DenseState::new_init_state(&shape);
        let mut iter = DenseOneBodyIterator::new(&state);

        assert!(&state.indices == &iter.start_state.indices);

        let (_, new_state) = iter.next().unwrap();
        assert!(vec![0, 1, 0] == new_state.indices);

        let (_, new_state) = iter.next().unwrap();
        assert!(vec![0, 2, 0] == new_state.indices);

        let (_, new_state) = iter.next().unwrap();
        assert!(vec![0, 3, 0] == new_state.indices);

        let (_, new_state) = iter.next().unwrap();
        assert!(vec![1, 0, 0] == new_state.indices);

        let (_, new_state) = iter.next().unwrap();
        assert!(vec![2, 0, 0] == new_state.indices);

        assert_eq!(iter.next().is_none(), true);
    }

    #[test]
    fn test_dense_one_body_iterator() {
        let shape = vec![3, 4, 1];
        let state = DenseState::new_init_state(&shape);

        for (sign, next_state) in DenseOneBodyIterator::new(&state) {
            assert!(sign == 1);

            let mut sum = 0;
            for (x, y) in state
                .indices
                .clone()
                .into_iter()
                .zip(next_state.indices.clone().into_iter())
            {
                if x != y {
                    sum += 1;
                }
            }

            assert!(sum == 1);
        }
    }
}
