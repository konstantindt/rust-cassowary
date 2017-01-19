use tableau::tables::Table;

pub fn get_leave_var_row_index(enter_var_index: usize, table: &Table) -> usize {
    let table_rows = table.get_rows();
    let last_column_index = table.get_column_names().len() - 1;
    let mut min_value = table_rows[0][last_column_index] / table_rows[0][enter_var_index];
    let mut colunm_index = 0;
    for i in 1..table_rows.len() - 1 { // We never pivot on the Z row
        let other_value = table_rows[i][last_column_index] / table_rows[i][enter_var_index];
        let old_min_value = min_value;
        min_value = old_min_value.min(other_value);
        // Did we assign a new max?
        if min_value != old_min_value {
            colunm_index = i;
        }
    }
    colunm_index
}
