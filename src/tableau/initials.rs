use std::collections::HashMap;
use Num;
use objective::functions::Function;
use objective::constraints::{Constraint, SystemOfConstraints};
use tableau::tables::Table;

pub fn get_initial_table_from(fun: &Function, constraints: &SystemOfConstraints) -> Table {
    let mut column_names: HashMap<String, usize> = HashMap::new();
    // We have number of rows equal to the number of regular constraints plus a
    // row for the function we are maxising or minimising.
    let mut num_rows: usize = 0;
    // We need the length of rows and we should be able to index into the table
    // with respect to a variable name.
    for constraint in constraints.system() {
        match constraint {
            &Constraint::Regular(ref ref_cell) => {
                num_rows += 1;
                let exp = ref_cell.borrow();
                for var in exp.lhs() {
                    insert_column_name_not_present(var.name().to_string(), &mut column_names);
                }
            }
            &Constraint::NonNegative(_) => continue,
        }
    }
    for var in fun.exp_max().borrow().lhs() {
        insert_column_name_not_present(var.name().to_string(), &mut column_names);
    }
    // ... and don't forget about the constant on the right.
    let map_len = column_names.len();
    column_names.insert("RHS".to_string(), map_len);

    let mut rows: Vec<Vec<Num>> = vec![vec![0.0; column_names.len()]; num_rows];
    // Populate the table
    let mut row_index = 0;
    for constraint in constraints.system() {
        match constraint {
            &Constraint::Regular(ref ref_cell) => {
                let exp = ref_cell.borrow();
                for var in exp.lhs() {
                    rows[row_index]
                        [*column_names.get(var.name())
                         .expect("get_initial_table_from: variable name key not present.")] =
                        var.get_data();
                }
                // ... and don't forget about the constant on the right.
                let last_column = rows[row_index].len() - 1;
                rows[row_index][last_column] = exp.rhs()[0].get_data();
                row_index += 1;
            }
            &Constraint::NonNegative(_) => continue,
        }
    }
    rows.push(get_row_for_function(fun, &column_names));

    Table::new(column_names, rows)
}

pub fn append_function(fun: &Function, to_table: &mut Table) {
    to_table.append_empty_column(fun.name().to_string());
    let row_to_append = get_row_for_function(fun, to_table.get_column_names());
    to_table.append_row(row_to_append);
    let old_num_fun_rows = to_table.get_num_fun_rows();
    to_table.set_num_fun_rows(old_num_fun_rows + 1);
}

fn get_row_for_function(fun: &Function, c_n: &HashMap<String, usize>) -> Vec<Num> {
    let fun_vars = fun.exp_max().borrow();
    let mut fun_row = vec![0.0; c_n.len()];
    for var in fun_vars.lhs() {
        fun_row[*c_n.get(var.name())
                .expect("get_row_for_function: variable name key not present.")] =
            var.get_data();
    }
    // Set the value of {Fun name} in the table.
    fun_row[c_n.len() - 1] = fun_vars.rhs()[0].get_data();
    fun_row
}

fn insert_column_name_not_present(name: String, into: &mut HashMap<String, usize>) {
    let map_len = into.len();
    into.entry(name).or_insert(map_len);
}
