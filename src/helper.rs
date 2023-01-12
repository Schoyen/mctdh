use ndarray::{Array, Array1, Array2};
use num::complex::Complex;

fn init_sd_state(start: usize, n: usize, l: usize) -> Vec<usize> {
    if start + n >= l {
        panic!("Too high start value, or too few orbitals `l` for `n` particles");
    }

    (start..(start + n)).collect()
}

fn next_sd_state(state: &Vec<usize>, n: usize, l: usize) -> Option<Vec<usize>> {
    let mut new_state = state.clone();
    let mut cursor = match n.checked_sub(1) {
        Some(c) => Some(c),
        None => panic!("n must be greater than zero"),
    };
    let mut dec = 0;

    while new_state[cursor.unwrap()] >= l - 1 - dec {
        cursor = match cursor?.checked_sub(1) {
            Some(c) => Some(c),
            None => return None, // Final state has been reached
        };
        dec += 1;
    }

    new_state[cursor.unwrap()] += 1;
    for c in (cursor.unwrap() + 1)..n {
        new_state[c] = new_state[cursor.unwrap()] + c - cursor.unwrap();
    }

    Some(new_state)
}

pub fn eval_sd_one_body_operator(
    c: &Array1<Complex<f64>>,
    h: &Array2<Complex<f64>>,
    n: usize,
) -> Array1<Complex<f64>> {
    let l = h.nrows();
    let mut c_new = Array::<Complex<f64>, _>::zeros(c.len());

    let mut state = init_sd_state(0, n, l);
    // TODO: c_i should be found from the compound indices formula
    let mut c_i = 0;

    loop {
        let mut val = Complex::new(0.0, 0.0);
        let mut an_sign = -1;

        for start in 0..2 {
            an_sign *= -1;

            for q_i in (start..state.len()).step_by(2) {
                let mut new_state = state.clone();
                let q = new_state.remove(q_i);

                // p < q
                let mut cr_sign = 1;
                for p in 0..q {
                    for q_ii in 0..q_i {}
                }

                // p == q
                val += h[[q, q]];

                // p > q
                for p in (q + 1)..l {}
            }
        }

        state = match next_sd_state(&state, n, l) {
            Some(x) => x,
            None => break,
        };
        // TODO: c_i should be found from the compound indices formula
        c_i += 1;
    }

    c_new
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array;

    #[test]
    fn test_init_sd_state() {
        let n = 4;
        let l = 11;

        let state = init_sd_state(0, n, l);
        for i in 0..state.len() {
            assert!(i == state[i]);
        }
    }

    #[test]
    fn test_next_sd_state() {
        let n = 3;
        let l = 10;

        let mut state = init_sd_state(0, n, l);

        for p in 0..l {
            for q in (p + 1)..l {
                for r in (q + 1)..l {
                    assert!(p == state[0] && q == state[1] && r == state[2]);
                    state = match next_sd_state(&state, n, l) {
                        Some(x) => x,
                        None => continue,
                    };
                }
            }
        }
    }

    #[test]
    fn test_break_eval_sd_one_body_operator() {
        let n = 3;
        let l = 10;
        let c = Array::<Complex<f64>, _>::zeros(l * (l - 1) * (l - 2) / (n * (n - 1) * (n - 2)));
        let h = Array::<Complex<f64>, _>::zeros((l, l));
        eval_sd_one_body_operator(&c, &h, n);
        assert!(true);
    }
}
