use std::collections::HashMap;
use objective::functions::Function;
use objective::constraints::{Constraint, SystemOfConstraints};
use tableau::tables::Table;

pub fn get_initial_table_from(fun: &Function, constraints: &SystemOfConstraints) -> Table {
    let mut column_names: HashMap<String, usize> = HashMap::new();
    // We have number of rows equal to the number of regular constraints plus a
    // row for the function we are maxising or minimising.
    let mut num_rows: usize = 1;
    // We need the length of rows and we should be able to index into the table
    // with respect to a variable name.
    for constraint in constraints.system() {
        match constraint {
            &Constraint::Regular(ref ref_cell) => {
                num_rows += 1;
                let exp = ref_cell.borrow();
                for var in exp.lhs() {
                    let map_len = column_names.len();
                    column_names.entry(var.name().to_string()).or_insert(map_len);
                }
            }
            &Constraint::NonNegative(_) => continue,
        }
    }
    for var in fun.exp_max().borrow().lhs() {
        let map_len = column_names.len();
        column_names.entry(var.name().to_string()).or_insert(map_len);
    }
    // ... and don't forget about the constant on the right.
    let map_len = column_names.len();
    column_names.insert("RHS".to_string(), map_len);
    let mut rows: Vec<Vec<f64>> = vec![vec![0.0; column_names.len()]; num_rows];
    // Populate the table
    let mut row_index = 0;
    for constraint in constraints.system() {
        match constraint {
            &Constraint::Regular(ref ref_cell) => {
                let exp = ref_cell.borrow();
                for var in exp.lhs() {
                    rows[row_index][*column_names.get(var.name()).unwrap()] = var.get_data();
                }
                // ... and don't forget about the constant on the right.
                let last_column = rows[row_index].len() - 1;
                rows[row_index][last_column] = exp.rhs()[0].get_data();
                row_index += 1;
            }
            &Constraint::NonNegative(_) => continue,
        }
    }
    let fun_vars = fun.exp_max().borrow();
    for var in fun_vars.lhs() {
        rows[row_index][*column_names.get(var.name()).unwrap()] = var.get_data();
    }
    // Set the value of Z to be initially 0 in the table.
    rows[row_index][map_len] = fun_vars.rhs()[0].get_data();
    Table::new(column_names, rows)
}
