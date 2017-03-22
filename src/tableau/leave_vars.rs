use tableau::tables::Table;

pub fn leave_var(enter_var_index: usize, table: &Table) -> usize {
    let table_rows = table.get_rows();
    let last_column_index = table.get_column_names().len() - 1;
    // Pick according to the smallest positive ratio of the entry in the
    // RHS column and the corresponding entry in pivot column.
    let mut row_index = 0;
    let mut ratio_current = table_rows[row_index][last_column_index] /
                            table_rows[row_index][enter_var_index];
    let mut i = 1;
    // Do not consider RHS of the function rows as we do not pivot on it.
    'down_columns: while i < table_rows.len() - table.get_num_fun_rows() {
        if ratio_current.is_sign_negative() {
            row_index = row_index + 1;
            ratio_current = table_rows[row_index][last_column_index] /
                            table_rows[row_index][enter_var_index];
            continue 'down_columns;
        }
        // Find the next positive ratio
        let mut ratio_next = table_rows[i][last_column_index] / table_rows[i][enter_var_index];
        while ratio_next.is_sign_negative() {
            i = i + 1;
            if i > table_rows.len() - table.get_num_fun_rows() {
                break 'down_columns;
            }
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
