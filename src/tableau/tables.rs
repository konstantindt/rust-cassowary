use std::collections::HashMap;
use std::result::Result;

pub struct Table {
    column_names: HashMap<String, usize>, // assume last column reserved
    rows: Vec<Vec<f64>>,
}

impl Table {
    pub fn new(c_n: HashMap<String, usize>, r: Vec<Vec<f64>>) -> Table {
        Table {
            rows: r,
            column_names: c_n,
        }
    }

    pub fn get_column_names(&self) -> &HashMap<String, usize> {
        &self.column_names
    }

    pub fn get_rows(&self) -> &Vec<Vec<f64>> {
        &self.rows
    }

    pub fn get_basic_solution(&self) -> Result<Vec<(String, f64)>, usize> {
        let mut basic_solution = Vec::with_capacity(self.column_names.len());
        // Note: ignore RHS column.
        'columns: for i in 0..self.column_names.len() - 1 {
            let mut one_entry_index = 0;
            let mut matched_one = false;
            // Find columns that have exactly one 1.0 and rest 0.0 values...
            'column_entries: for j in 0..self.rows.len() {
                match (matched_one, self.rows[j][i]) {
                    (false, 1.0) | (false, -1.0) => {
                        one_entry_index = j;
                        matched_one = true;
                    }
                    (true, 0.0) | (false, 0.0) => continue 'column_entries,
                    _ => {
                        // ... if this is not the case then the value of this
                        // variables in the basic solution is 0.0.
                        basic_solution.push((get_name_of_index(&self.column_names, i).unwrap(),
                                            0.0));
                        continue 'columns;
                    }
                }
            }
            // ... and when we find a basic variable calculate its value but
            // watch out as it might be all zeros so check if we matched a 1.0.
            if matched_one {
                let basic_variable_value = self.rows[one_entry_index][i] *
                                           self.rows[one_entry_index][self.column_names.len() - 1];
                // If the basic variable turns out negative that this solution
                // is not feasable...
                if basic_variable_value.is_sign_negative() {
                    // ... report the row where it happened.
                    return Err(one_entry_index);
                } else {
                    // ... if not continue generating the solution.
                    basic_solution.push((get_name_of_index(&self.column_names, i).unwrap(),
                                         basic_variable_value));
                }
            } else {
                basic_solution.push((get_name_of_index(&self.column_names, i).unwrap(), 0.0));
            }
        }
        // If we got here then solution is feasable so return it.
        Ok(basic_solution)
    }

    pub fn is_solution_optimal(&self) -> bool {
        for i in 0..self.column_names.len() - 1 {
            if self.rows[self.rows.len() - 1][i].is_sign_negative() {
                return false;
            }
        }
        true
    }

    pub fn sub_cell(&mut self, row_index: usize, colunm_index: usize, by: f64) {
        self.rows[row_index][colunm_index] = self.rows[row_index][colunm_index] - by;
    }

    pub fn div_cell(&mut self, row_index: usize, colunm_index: usize, by: f64) {
        self.rows[row_index][colunm_index] = self.rows[row_index][colunm_index] / by;
    }
}

fn get_name_of_index(c_n: &HashMap<String, usize>, index: usize) -> Result<String, &str> {
    for (key, val) in c_n.iter() {
        if *val == index {
            return Ok(key.clone());
        }
    }
    Err("Name not found for index given.")
}
