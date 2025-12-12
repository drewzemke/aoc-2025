pub mod puzzle10a;
pub mod puzzle10b;

#[derive(Debug)]
pub struct Machine {
    num_lights: usize,
    light_target: Vec<bool>,
    buttons: Vec<Vec<bool>>,
    #[expect(dead_code)]
    joltage_requirements: Vec<usize>,
}

impl Machine {
    // [.##.]
    fn parse_lights(input: &str) -> (usize, Vec<bool>) {
        let target = input[1..input.len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect();
        (input.len() - 2, target)
    }

    // (3) (1,3) (2) (2,3) (0,2) (0,1)
    fn parse_button(input: &str, num_lights: usize) -> Vec<bool> {
        let nums: Vec<usize> = input[1..input.len() - 1]
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        (0..num_lights).map(|n| nums.contains(&n)).collect()
    }

    // {3,5,4,7}
    fn parse_reqs(input: &str) -> Vec<usize> {
        input[1..input.len() - 1]
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect()
    }

    pub fn parse(input: &str) -> Self {
        let mut chunks = input.split(' ');

        let (num_lights, light_target) = Self::parse_lights(chunks.next().unwrap());

        let buttons = chunks
            .clone()
            .take_while(|s| s.starts_with('('))
            .map(|s| Self::parse_button(s, num_lights))
            .collect();

        let joltage_requirements = Self::parse_reqs(chunks.next_back().unwrap());

        Self {
            num_lights,
            light_target,
            buttons,
            joltage_requirements,
        }
    }

    /// yuppp we're doing row reductions babyyy
    ///
    /// returns (columns, rhs) with columns unsorted
    fn row_reduce(&self) -> (Vec<Vec<bool>>, Vec<bool>) {
        let num_cols = self.buttons.len();

        // join the buttons an target light together
        let mut matrix = self.buttons.clone();
        matrix.push(self.light_target.clone());

        // print_mat(&matrix);

        let mut pivot_rows = Vec::new();

        // algorithm:
        //
        // for every p from 0 to num_cols
        //   find a row with a 1 in column p
        //   if found,
        //     add that row to every other row that also has a 1
        //     in the same column (btw addition is mod 2)
        //   if not found, move on
        for p in 0..num_cols {
            // println!("pivot: {p}");

            let pivot_row_idx = matrix[p]
                .iter()
                .enumerate()
                .position(|(idx, b)| *b && !pivot_rows.contains(&idx));

            if let Some(pivot_row_idx) = pivot_row_idx {
                pivot_rows.push(pivot_row_idx);

                // println!("pivot row idx: {pivot_row_idx}");

                // add this row to every other row with a 1 in col p
                let target_row_idxs = matrix[p]
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, b)| (idx != pivot_row_idx && *b).then_some(idx))
                    // FIXME: don't need to collect here, can just do a for_each
                    .collect::<Vec<_>>();

                // println!("target rows: {target_row_idxs:?}");

                for row_idx in target_row_idxs {
                    for col in &mut matrix {
                        col[row_idx] ^= col[pivot_row_idx];
                    }
                }
            }

            // print_mat(&matrix);
        }

        // pull out the right hand side again before we return
        let rhs = matrix.remove(num_cols);
        (matrix, rhs)
    }

    /// computes of the solution of the linear system determined by the
    /// buttons (basis vectors) and lights (right-hand-side) that has the
    /// smallest magnitude
    pub fn min_z2_soln(&self) -> Vec<bool> {
        let (cols, rhs) = self.row_reduce();

        // print_mat(&cols);
        // print_mat(std::slice::from_ref(&rhs));

        // determine free variablee by checking which columns don't
        // have exactly one 1 in them
        let free_vars = (0..cols.len())
            .filter(|col| cols[*col].iter().filter(|b| **b).count() != 1)
            .collect::<Vec<_>>();

        let free_var_combos = all_combos(&free_vars);

        // using all combinations of free variables,
        // compute solution, return the one with the smallest magnitude
        free_var_combos
            .into_iter()
            .map(|free_vars| {
                let mut sol = vec![false; cols.len()];

                // first put in the free variable values
                for idx in &free_vars {
                    sol[*idx] = true;
                }

                // in each row, the position of the leading 1 determines the
                // position that is set in the solution vector. 1's to the right of the leading 1
                // will correspond to free vars, so xor them them into the solution vector entry
                // finally, xor in the corresponding rhs entry to this row
                for row in 0..self.num_lights {
                    let lead_idx = cols.iter().position(|col| col[row]);
                    if let Some(lead_idx) = lead_idx {
                        #[expect(clippy::needless_range_loop)]
                        for col in lead_idx + 1..cols.len() {
                            if free_vars.contains(&col) {
                                sol[lead_idx] ^= cols[col][row];
                            }
                        }

                        sol[lead_idx] ^= rhs[row];
                    }
                }

                sol
            })
            .min_by_key(|v| v.iter().filter(|b| **b).count())
            .unwrap()
    }
}

fn all_combos(arr: &[usize]) -> Vec<Vec<usize>> {
    if arr.is_empty() {
        Vec::from([Vec::new()])
    } else if arr.len() == 1 {
        Vec::from([arr.to_vec(), Vec::new()])
    } else {
        let combos = all_combos(&arr[1..]);
        combos
            .clone()
            .into_iter()
            .map(|mut v| {
                v.push(arr[0]);
                v
            })
            .chain(combos)
            .collect()
    }
}

#[allow(dead_code)]
fn print_mat(mat: &[Vec<bool>]) {
    for i in 0..mat[0].len() {
        #[expect(clippy::needless_range_loop)]
        for j in 0..mat.len() {
            let v = if mat[j][i] { '1' } else { '0' };
            print!("{v} ");
        }
        println!();
    }
}
