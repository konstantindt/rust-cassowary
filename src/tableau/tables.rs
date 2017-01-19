use std::collections::HashMap;

pub struct Table {
    column_names: HashMap<String, usize>,
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
}
