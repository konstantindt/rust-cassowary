use tableau::tables::Table;

pub fn get_enter_var_column_index(table: &Table) -> usize {
    let table_rows = table.get_rows();
    let last_row_index = table_rows.len() - 1;
    let mut max_value = table_rows[last_row_index][0];
    let mut colunm_index = 0;
    for i in 1..table_rows[last_row_index].len() - 1 {
        let other_value = table_rows[last_row_index][i];
        let old_max_value = max_value;
        max_value = old_max_value.max(other_value);
        // Did we assign a new max?
        if max_value != old_max_value {
            colunm_index = i;
        }
    }
    colunm_index
}
