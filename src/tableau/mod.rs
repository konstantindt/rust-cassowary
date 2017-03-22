pub mod tables;
pub mod initials;
pub mod enter_vars;
pub mod leave_vars;
pub mod pivots;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::cell::RefCell;
    use math::variables::{new_var, new_const};
    use math::relationships::Relationship;
    use math::expressions::Expression;
    use objective::problems::ProblemType;
    use objective::functions::Function;
    use objective::constraints::{Constraint, SystemOfConstraints};
    use objective::solvers::{transform_constraint_rels_to_eq, rearrange_fun_eq_zero};
    use tableau::tables::Table;
    use tableau::initials::get_initial_table_from;
    use tableau::enter_vars::{enter_var_pivot_optimal, enter_var_pivot_feasible};
    use tableau::leave_vars::leave_var;
    use tableau::pivots::pivot_around;

    #[test]
    fn can_create_tables() {
        let mut column_names: HashMap<String, usize> = HashMap::new();
        column_names.insert("x".to_string(), 0);
        column_names.insert("y".to_string(), 1);
        let rows = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let table = Table::new(column_names, rows);
        let table_header = table.get_column_names();
        let table_rows = table.get_rows();
        assert!(table_header.contains_key("x"));
        assert!(table_header.contains_key("y"));
        assert_eq!(2, table_header.len());
        assert_eq!(0, *table_header.get("x").unwrap());
        assert_eq!(1, *table_header.get("y").unwrap());
        assert_eq!(1.0, table_rows[0][0]);
        assert_eq!(2.0, table_rows[0][1]);
        assert_eq!(3.0, table_rows[1][0]);
        assert_eq!(4.0, table_rows[1][1]);
        assert_eq!(5.0, table_rows[2][0]);
        assert_eq!(6.0, table_rows[2][1]);
    }

    #[test]
    fn can_sub_cell() {
        let mut column_names: HashMap<String, usize> = HashMap::new();
        column_names.insert("x".to_string(), 0);
        column_names.insert("y".to_string(), 1);
        let rows = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let mut table = Table::new(column_names, rows);
        table.sub_cell(1, 1, 2.0);
        let table_header = table.get_column_names();
        let table_rows = table.get_rows();
        assert!(table_header.contains_key("x"));
        assert!(table_header.contains_key("y"));
        assert_eq!(2, table_header.len());
        assert_eq!(0, *table_header.get("x").unwrap());
        assert_eq!(1, *table_header.get("y").unwrap());
        assert_eq!(1.0, table_rows[0][0]);
        assert_eq!(2.0, table_rows[0][1]);
        assert_eq!(3.0, table_rows[1][0]);
        assert_eq!(2.0, table_rows[1][1]);
        assert_eq!(5.0, table_rows[2][0]);
        assert_eq!(6.0, table_rows[2][1]);
    }

    #[test]
    fn can_div_cell() {
        let mut column_names: HashMap<String, usize> = HashMap::new();
        column_names.insert("x".to_string(), 0);
        column_names.insert("y".to_string(), 1);
        let rows = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let mut table = Table::new(column_names, rows);
        table.div_cell(2, 1, 2.0);
        let table_header = table.get_column_names();
        let table_rows = table.get_rows();
        assert!(table_header.contains_key("x"));
        assert!(table_header.contains_key("y"));
        assert_eq!(2, table_header.len());
        assert_eq!(0, *table_header.get("x").unwrap());
        assert_eq!(1, *table_header.get("y").unwrap());
        assert_eq!(1.0, table_rows[0][0]);
        assert_eq!(2.0, table_rows[0][1]);
        assert_eq!(3.0, table_rows[1][0]);
        assert_eq!(4.0, table_rows[1][1]);
        assert_eq!(5.0, table_rows[2][0]);
        assert_eq!(3.0, table_rows[2][1]);
    }

    #[test]
    fn can_get_basic_solution() {
        let mut column_names1: HashMap<String, usize> = HashMap::new();
        column_names1.insert("P".to_string(), 0);
        column_names1.insert("x".to_string(), 1);
        column_names1.insert("y".to_string(), 2);
        column_names1.insert("s".to_string(), 3);
        column_names1.insert("t".to_string(), 4);
        column_names1.insert("u".to_string(), 5);
        column_names1.insert("Value".to_string(), 6);
        let table1_rows = vec![vec![0.0, 0.0, 0.0, 1.0 / 6.0, 0.0, 2.0, 55.0],
                               vec![0.0, 0.0, 1.0, 1.0 / 3.0, 0.0, -1.0, 10.0],
                               vec![0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, -3.0, 5.0],
                               vec![0.0, 1.0, 0.0, -1.0 / 3.0, 0.0, 0.5, 10.0]];
        let table1 = Table::new(column_names1, table1_rows);
        assert_eq!(vec![("x".to_string(), 10.0),
                        ("y".to_string(), 10.0),
                        ("t".to_string(), 5.0)],
                   table1.get_basic_solution().unwrap());

        let mut column_names2: HashMap<String, usize> = HashMap::new();
        column_names2.insert("P".to_string(), 0);
        column_names2.insert("x".to_string(), 1);
        column_names2.insert("y".to_string(), 2);
        column_names2.insert("s".to_string(), 3);
        column_names2.insert("t".to_string(), 4);
        column_names2.insert("u".to_string(), 5);
        column_names2.insert("Value".to_string(), 6);
        let table2_rows = vec![vec![0.0, 0.0, 0.0, 1.0 / 6.0, 0.0, 2.0, 55.0],
                               vec![0.0, 0.0, -1.0, 1.0 / 3.0, 0.0, -1.0, 10.0],
                               vec![0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, -3.0, 5.0],
                               vec![0.0, 1.0, 0.0, -1.0 / 3.0, 0.0, 0.5, 10.0]];
        let table2 = Table::new(column_names2, table2_rows);
        assert_eq!((1, 2), table2.get_basic_solution().err().unwrap());
    }

    #[test]
    fn can_check_is_solution_optimal() {
        let mut column_names1: HashMap<String, usize> = HashMap::new();
        column_names1.insert("P".to_string(), 0);
        column_names1.insert("x".to_string(), 1);
        column_names1.insert("y".to_string(), 2);
        column_names1.insert("s".to_string(), 3);
        column_names1.insert("t".to_string(), 4);
        column_names1.insert("u".to_string(), 5);
        column_names1.insert("Value".to_string(), 6);
        let table1_rows = vec![vec![0.0, 0.0, 0.0, 1.0 / 6.0, 0.0, 2.0, 55.0],
                               vec![0.0, 0.0, -1.0, 1.0 / 3.0, 0.0, -1.0, 10.0],
                               vec![0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, -3.0, 5.0],
                               vec![0.0, 1.0, 0.0, -1.0 / 3.0, 0.0, 0.5, 10.0]];
        let table1 = Table::new(column_names1, table1_rows);
        assert_eq!(false, table1.is_solution_optimal());

        let mut column_names2: HashMap<String, usize> = HashMap::new();
        column_names2.insert("P".to_string(), 0);
        column_names2.insert("x".to_string(), 1);
        column_names2.insert("y".to_string(), 2);
        column_names2.insert("s".to_string(), 3);
        column_names2.insert("t".to_string(), 4);
        column_names2.insert("u".to_string(), 5);
        column_names2.insert("Value".to_string(), 6);
        let table2_rows = vec![vec![0.0, 0.0, 0.0, 1.0 / 6.0, 0.0, 2.0, 55.0],
                               vec![0.0, 0.0, -1.0, 1.0 / 3.0, 0.0, -1.0, 10.0],
                               vec![0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, -3.0, 5.0],
                               vec![0.0, 1.0, 0.0, 1.0 / 3.0, 0.0, 0.5, 10.0]];
        let table2 = Table::new(column_names2, table2_rows);
        assert_eq!(true, table2.is_solution_optimal());

        let mut column_names3: HashMap<String, usize> = HashMap::new();
        column_names3.insert("P".to_string(), 0);
        column_names3.insert("x".to_string(), 1);
        column_names3.insert("y".to_string(), 2);
        column_names3.insert("s".to_string(), 3);
        column_names3.insert("t".to_string(), 4);
        column_names3.insert("u".to_string(), 5);
        column_names3.insert("Value".to_string(), 6);
        let table3_rows = vec![vec![0.0, 0.0, 0.0, 1.0 / 6.0, 0.0, 2.0, 55.0],
                               vec![0.0, 0.0, -1.0, 1.0 / 3.0, 0.0, -1.0, 10.0],
                               vec![0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, -3.0, 5.0],
                               vec![0.0, 1.0, 0.0, 1.0 / 3.0, 0.0, 0.5, -10.0]];
        let table3 = Table::new(column_names3, table3_rows);
        assert_eq!(true, table3.is_solution_optimal());
    }

    #[test]
    fn can_create_initial_tableau() {
        let e1 =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x1", 6.0), new_var("x2", 14.0), new_var("x3", 13.0)]);
        let mut f = Function::new(e1, ProblemType::MAX);
        rearrange_fun_eq_zero(&mut f);
        let e2 = Expression::new(vec![new_var("x1", 0.5), new_var("x2", 2.0), new_var("x3", 1.0)],
                                 Relationship::LEQ,
                                 vec![new_const("Metalworking (days)", 24.0)]);
        let e3 = Expression::new(vec![new_var("x1", 1.0), new_var("x2", 2.0), new_var("x3", 4.0)],
                                 Relationship::LEQ,
                                 vec![new_const("Woodworking (days)", 60.0)]);
        let c1 = Constraint::Regular(RefCell::new(e2));
        let c2 = Constraint::Regular(RefCell::new(e3));
        let c3 = Constraint::NonNegative(new_var("x1", 1.0));
        let c4 = Constraint::NonNegative(new_var("x2", 1.0));
        let c5 = Constraint::NonNegative(new_var("x3", 1.0));
        let system = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5]);
        transform_constraint_rels_to_eq(&system);
        let table = get_initial_table_from(&f, &system);
        let table_header = table.get_column_names();
        let table_rows = table.get_rows();
        assert_eq!(7, table_header.len());
        assert_eq!(3, table_rows.len());
        assert!(table_header.contains_key("x1"));
        assert!(table_header.contains_key("x2"));
        assert!(table_header.contains_key("x3"));
        assert!(table_header.contains_key("sl1"));
        assert!(table_header.contains_key("sl2"));
        assert!(table_header.contains_key("Z"));
        assert!(table_header.contains_key("RHS"));
        assert_eq!(0, *table_header.get("x1").unwrap());
        assert_eq!(1, *table_header.get("x2").unwrap());
        assert_eq!(2, *table_header.get("x3").unwrap());
        assert_eq!(3, *table_header.get("sl1").unwrap());
        assert_eq!(4, *table_header.get("sl2").unwrap());
        assert_eq!(5, *table_header.get("Z").unwrap());
        assert_eq!(6, *table_header.get("RHS").unwrap());
        assert_eq!(vec![0.5, 2.0, 1.0, 1.0, 0.0, 0.0, 24.0], table_rows[0]);
        assert_eq!(vec![1.0, 2.0, 4.0, 0.0, 1.0, 0.0, 60.0], table_rows[1]);
        assert_eq!(vec![-6.0, -14.0, -13.0, 0.0, 0.0, 1.0, 0.0], table_rows[2]);
    }

    #[test]
    fn can_enter_var_pivot_optimal() {
        let mut column_names: HashMap<String, usize> = HashMap::new();
        column_names.insert("x1".to_string(), 0);
        column_names.insert("x2".to_string(), 1);
        column_names.insert("x3".to_string(), 2);
        column_names.insert("s1".to_string(), 3);
        column_names.insert("s2".to_string(), 4);
        column_names.insert("RHS".to_string(), 5);
        let table_rows = vec![vec![0.5, 2.0, 1.0, 1.0, 0.0, 24.0],
                              vec![1.0, 2.0, 4.0, 0.0, 1.0, 60.0],
                              vec![-6.0, -14.0, -13.0, 0.0, 0.0, 0.0]];
        let table = Table::new(column_names, table_rows);
        assert_eq!(1, enter_var_pivot_optimal(&table));
    }

    #[test]
    fn can_enter_var_pivot_feasible() {
        let mut column_names1: HashMap<String, usize> = HashMap::new();
        column_names1.insert("x1".to_string(), 0);
        column_names1.insert("x2".to_string(), 1);
        column_names1.insert("x3".to_string(), 2);
        column_names1.insert("s1".to_string(), 3);
        column_names1.insert("s2".to_string(), 4);
        column_names1.insert("RHS".to_string(), 5);
        column_names1.insert("Value".to_string(), 6);
        let table1_rows = vec![vec![0.5, 2.0, 1.0, 1.0, 0.0, 24.0],
                               vec![-1.0, 0.0, 4.0, 0.0, 1.0, 60.0],
                               vec![-14.0, -6.0, -13.0, 0.0, 0.0, 0.0]];
        let table1 = Table::new(column_names1, table1_rows);
        assert_eq!(2, enter_var_pivot_feasible(&table1, 1, 4).unwrap());

        let mut column_names2: HashMap<String, usize> = HashMap::new();
        column_names2.insert("x1".to_string(), 0);
        column_names2.insert("x2".to_string(), 1);
        column_names2.insert("x3".to_string(), 2);
        column_names2.insert("s1".to_string(), 3);
        column_names2.insert("s2".to_string(), 4);
        column_names2.insert("RHS".to_string(), 5);
        column_names2.insert("Value".to_string(), 6);
        let table2_rows = vec![vec![0.5, 2.0, 1.0, 1.0, 0.0, 24.0],
                               vec![-1.0, 0.0, -4.0, 0.0, 1.0, 60.0],
                               vec![-14.0, -6.0, -13.0, 0.0, 0.0, 0.0]];
        let table2 = Table::new(column_names2, table2_rows);
        assert_eq!(true, enter_var_pivot_feasible(&table2, 1, 4).is_none());
    }

    #[test]
    fn can_leave_var_row_index() {
        let mut column_names1: HashMap<String, usize> = HashMap::new();
        column_names1.insert("x1".to_string(), 0);
        column_names1.insert("x2".to_string(), 1);
        column_names1.insert("x3".to_string(), 2);
        column_names1.insert("s1".to_string(), 3);
        column_names1.insert("s2".to_string(), 4);
        column_names1.insert("RHS".to_string(), 5);
        let table1_rows = vec![vec![0.5, 2.0, 1.0, 1.0, 0.0, 24.0],
                               vec![1.0, 2.0, 4.0, 0.0, 1.0, 60.0],
                               vec![-14.0, -6.0, -13.0, 0.0, 0.0, 0.0]];
        let table1 = Table::new(column_names1, table1_rows);
        let enter_var_index1 = enter_var_pivot_optimal(&table1);
        assert_eq!(0, leave_var(enter_var_index1, &table1));

        let mut column_names2: HashMap<String, usize> = HashMap::new();
        column_names2.insert("x1".to_string(), 0);
        column_names2.insert("x2".to_string(), 1);
        column_names2.insert("x3".to_string(), 2);
        column_names2.insert("s1".to_string(), 3);
        column_names2.insert("s2".to_string(), 4);
        column_names2.insert("RHS".to_string(), 5);
        let table2_rows = vec![vec![-0.5, 2.0, 1.0, 1.0, 0.0, 24.0],
                               vec![1.0, 2.0, 4.0, 0.0, 1.0, 60.0],
                               vec![-14.0, -6.0, -13.0, 0.0, 0.0, 0.0]];
        let table2 = Table::new(column_names2, table2_rows);
        let enter_var_index2 = enter_var_pivot_optimal(&table2);
        assert_eq!(1, leave_var(enter_var_index2, &table2));

        let mut column_names3: HashMap<String, usize> = HashMap::new();
        column_names3.insert("x1".to_string(), 0);
        column_names3.insert("x2".to_string(), 1);
        column_names3.insert("x3".to_string(), 2);
        column_names3.insert("s1".to_string(), 3);
        column_names3.insert("s2".to_string(), 4);
        column_names3.insert("RHS".to_string(), 5);
        let table3_rows = vec![vec![0.5, 2.0, 1.0, 1.0, 0.0, 24.0],
                               vec![-1.0, 2.0, 4.0, 0.0, 1.0, 60.0],
                               vec![-1.0, 2.0, 4.0, 0.0, 1.0, 60.0],
                               vec![1.0, 2.0, 4.0, 0.0, 1.0, 47.0],
                               vec![-14.0, -6.0, -13.0, 0.0, 0.0, 0.0]];
        let table3 = Table::new(column_names3, table3_rows);
        let enter_var_index3 = enter_var_pivot_optimal(&table3);
        assert_eq!(3, leave_var(enter_var_index3, &table3));
    }

    #[test]
    fn can_pivot_around() {
        let mut column_names: HashMap<String, usize> = HashMap::new();
        column_names.insert("x1".to_string(), 0);
        column_names.insert("x2".to_string(), 1);
        column_names.insert("s1".to_string(), 2);
        column_names.insert("s2".to_string(), 3);
        column_names.insert("s3".to_string(), 4);
        column_names.insert("Value".to_string(), 5);
        let table_rows = vec![vec![3.0, 6.0, 1.0, 0.0, 0.0, 90.0],
                              vec![2.0, 1.0, 0.0, 1.0, 0.0, 35.0],
                              vec![1.0, 1.0, 0.0, 0.0, 1.0, 20.0],
                              vec![-2.5, -3.0, 0.0, 0.0, 0.0, 0.0]];
        let mut table = Table::new(column_names, table_rows);
        pivot_around(1, 0, &mut table);
        let table_rows = table.get_rows();
        assert_eq!(vec![0.5, 1.0, (1.0 / 6.0), 0.0, 0.0, 15.0], table_rows[0]);
        assert_eq!(vec![1.5, 0.0, -(1.0 / 6.0), 1.0, 0.0, 20.0], table_rows[1]);
        assert_eq!(vec![0.5, 0.0, -(1.0 / 6.0), 0.0, 1.0, 5.0], table_rows[2]);
        assert_eq!(vec![-1.0, 0.0, 0.5, 0.0, 0.0, 45.0], table_rows[3]);
    }
}
