use tableau::tables::Table;

pub fn enter_var_pivot_optimal(table: &Table) -> usize {
    let table_rows = table.get_rows();
    let last_row_index = table_rows.len() - 1;
    // Select the most negative cell in the objective function row.
    let mut column_index = 0;
    for i in 0..table_rows[last_row_index].len() - 1 {
        if table_rows[last_row_index][i] < table_rows[last_row_index][column_index] {
            column_index = i;
        }
    }
    column_index
}

pub fn enter_var_pivot_feasible(table: &Table,
                                row: usize,
                                begin_column: usize)
                                -> Option<usize> {
    let table_rows = table.get_rows();
    // Select the positive cell furthest to the left.
    for i in 0..begin_column {
        if table_rows[row][i].is_sign_positive() && table_rows[row][i] > 0.0 {
            return Some(i);
        }
    }
    None
}
