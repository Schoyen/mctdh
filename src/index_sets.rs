pub trait IndexSet {
    fn get_one_neighbor(&self) -> Option<(i64, Self)>
    where
        Self: Sized;
    fn get_two_neighbor(&self) -> Option<(i64, Self)>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct DenseState<'a> {
    shape: &'a Vec<usize>,
    indices: Vec<usize>,
    compound: usize,
}

pub struct DenseIterator<'a> {
    state: &'a DenseState<'a>,
    ind_diff: usize,
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

impl<'a> Iterator for DenseIterator<'a> {
    type Item = (i64, DenseState<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        Some((1, DenseState::new_init_state(self.state.shape)))
    }
}

impl<'a> IndexSet for DenseState<'a> {
    fn get_one_neighbor(&self) -> Option<(i64, Self)> {
        // This is wrong!
        let mut carry = 1;
        let mut indices = self.indices.clone();

        if carry == 1 {
            return None;
        }

        for i in (0..self.indices.len()).rev() {
            indices[i] = (self.indices[i] + carry) % self.shape[i];
            carry = (self.indices[i] + carry) / self.shape[i];
        }

        if carry != 0 {
            return None;
        }

        Some((1, DenseState::new_from_indices(indices, self.shape)))
    }

    fn get_two_neighbor(&self) -> Option<(i64, Self)> {
        // This is wrong!
        let mut carry = 1;
        let mut indices = self.indices.clone();
        if carry == 1 {
            return None;
        }

        for i in (0..self.indices.len()).rev() {
            indices[i] = (self.indices[i] + carry) % self.shape[i];
            carry = (self.indices[i] + carry) / self.shape[i];
        }

        if carry != 0 {
            return None;
        }

        Some((1, DenseState::new_from_indices(indices, self.shape)))
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
    fn test_get_one_neighbor_dense() {
        let shape = vec![2, 2, 4, 5];
        let mut counter = 0;

        let mut state = DenseState::new_init_state(&shape);

        loop {
            let (sign, neighbor_state) = match state.get_one_neighbor() {
                Some((s, n)) => (s, n),
                None => break,
            };

            assert!(sign == 1);
            // let mut diff
        }
    }
}
