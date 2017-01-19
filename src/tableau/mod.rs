pub mod tables;
pub mod initials;

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
    use objective::solvers::transform_leq_rels;
    use tableau::tables::Table;
    use tableau::initials::get_initial_table_from;

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
    fn can_create_initial_tableau() {
        let e = Expression::new(vec![new_var("Z", 1.0)],
                                Relationship::EQ,
                                vec![new_var("x1", 6.0), new_var("x2", 14.0), new_var("x3", 13.0)]);
        let f = Function::new(e, ProblemType::MAX);
        let e1 = Expression::new(vec![new_var("x1", 0.5), new_var("x2", 2.0), new_var("x3", 1.0)],
                                 Relationship::LEQ,
                                 vec![new_const("Metalworking (days)", 24.0)]);
        let e2 = Expression::new(vec![new_var("x1", 1.0), new_var("x2", 2.0), new_var("x3", 4.0)],
                                 Relationship::LEQ,
                                 vec![new_const("Woodworking (days)", 60.0)]);
        let c1 = Constraint::Regular(RefCell::new(e1));
        let c2 = Constraint::Regular(RefCell::new(e2));
        let c3 = Constraint::NonNegative(new_var("x1", 1.0));
        let c4 = Constraint::NonNegative(new_var("x2", 1.0));
        let c5 = Constraint::NonNegative(new_var("x3", 1.0));
        let system = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5]);
        transform_leq_rels(&system);
        let table = get_initial_table_from(&f, &system);
        let table_header = table.get_column_names();
        let table_rows = table.get_rows();
        assert_eq!(6, table_header.len());
        assert_eq!(3, table_rows.len());
        assert!(table_header.contains_key("x1"));
        assert!(table_header.contains_key("x2"));
        assert!(table_header.contains_key("x3"));
        assert!(table_header.contains_key("s1"));
        assert!(table_header.contains_key("s2"));
        assert!(table_header.contains_key("RHS"));
        assert_eq!(0, *table_header.get("x1").unwrap());
        assert_eq!(1, *table_header.get("x2").unwrap());
        assert_eq!(2, *table_header.get("x3").unwrap());
        assert_eq!(3, *table_header.get("s1").unwrap());
        assert_eq!(4, *table_header.get("s2").unwrap());
        assert_eq!(5, *table_header.get("RHS").unwrap());
        assert_eq!(vec![0.5, 2.0, 1.0, 1.0, 0.0, 24.0], table_rows[0]);
        assert_eq!(vec![1.0, 2.0, 4.0, 0.0, 1.0, 60.0], table_rows[1]);
        assert_eq!(vec![6.0, 14.0, 13.0, 0.0, 0.0, 0.0], table_rows[2]);
    }
}
