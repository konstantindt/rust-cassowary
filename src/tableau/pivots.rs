use tableau::tables::Table;

pub fn pivot_around(enter_var_index: usize, leave_var_index: usize, table: &mut Table) {
    let row_len = table.get_rows()[leave_var_index].len();
    let pivot_value = table.get_rows()[leave_var_index][enter_var_index];
    // Make all cells in pivot cell's column value 0.0
    for row_index in 0..table.get_rows().len() {
        if row_index != leave_var_index {
            let value_corres_pivot_column = table.get_rows()[row_index][enter_var_index];
            for i in 0..row_len {
                // value in line of pivot corresponding column of current cell *
                // value in current line corresponding pivot cell column
                let numerator = table.get_rows()[leave_var_index][i] *
                                value_corres_pivot_column;
                table.sub_cell(row_index, i, numerator / pivot_value);
            }
        }
    }
    // Make pivot cell value 1.0
    if pivot_value != 1.0 {
        for i in 0..row_len {
            table.div_cell(leave_var_index, i, pivot_value);
        }
    }
}
