use std::collections::HashMap;

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

    pub fn sub_cell(&mut self, row_index: usize, colunm_index: usize, by: f64) {
        self.rows[row_index][colunm_index] = self.rows[row_index][colunm_index] - by;
    }

    pub fn div_cell(&mut self, row_index: usize, colunm_index: usize, by: f64) {
        self.rows[row_index][colunm_index] = self.rows[row_index][colunm_index] / by;
    }
}
