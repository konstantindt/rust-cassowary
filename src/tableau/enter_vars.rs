use tableau::tables::Table;

pub fn get_enter_var_column_index(table: &Table) -> usize {
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
