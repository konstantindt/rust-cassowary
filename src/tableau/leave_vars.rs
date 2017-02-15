use tableau::tables::Table;

pub fn get_leave_var_row_index(enter_var_index: usize, table: &Table) -> usize {
    let table_rows = table.get_rows();
    let last_column_index = table.get_column_names().len() - 1;
    // Pick according to the smallest positive ratio of the entry in the
    // RHS column and the corresponding entry in pivot column.
    let mut row_index = 0;
    let mut ratio_current = table_rows[row_index][last_column_index] /
                            table_rows[row_index][enter_var_index];
    let mut i = 1;
    while i < table_rows.len() - 1 { // Only consider rows with non-basic variables.
        if ratio_current.is_sign_negative() {
            row_index = row_index + 1;
            ratio_current = table_rows[row_index][last_column_index] /
                            table_rows[row_index][enter_var_index];
            continue;
        }
        // Find the next positive ratio
        let mut ratio_next = table_rows[i][last_column_index] / table_rows[i][enter_var_index];
        while ratio_next.is_sign_negative() {
            i = i + 1;
            ratio_next = table_rows[i][last_column_index] / table_rows[i][enter_var_index];
        }
        if ratio_next < ratio_current {
            row_index = i;
            ratio_current = table_rows[row_index][last_column_index] /
                            table_rows[row_index][enter_var_index];
        }
        i = i + 1;
    }
    row_index
}
