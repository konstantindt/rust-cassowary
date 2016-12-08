pub mod problems;
pub mod functions;
pub mod constraints;
pub mod solvers;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use math::variables::{AbstVar, new_var, new_const};
    use math::relationships::Relationship;
    use math::expressions::Expression;
    use objective::problems::ProblemType;
    use objective::functions::Function;
    use objective::constraints::Constraint;
    use objective::constraints::SystemOfConstraints;
    use objective::solvers::{transform_geq_rels, transform_leq_rels};

    #[test]
    fn can_create_problem_types() {
        let p_t_max: ProblemType = ProblemType::MAX;
        assert_eq!(ProblemType::MAX, p_t_max);
        let p_t_min: ProblemType = ProblemType::MIN;
        assert_eq!(ProblemType::MIN, p_t_min);
    }

    #[test]
    fn can_create_functions() {
        let e: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        let f: Function = Function::new(e, ProblemType::MAX);
        assert_eq!(ProblemType::MAX, *f.p_type());
        assert_eq!("Z", f.exp().lhs()[0].name());
        assert_eq!(1.0, f.exp().lhs()[0].coefficient());
        assert_eq!(Relationship::EQ, *f.exp().rel());
        assert_eq!("x", f.exp().rhs()[0].name());
        assert_eq!(2.0, f.exp().rhs()[0].coefficient());
        assert_eq!("y", f.exp().rhs()[1].name());
        assert_eq!(3.0, f.exp().rhs()[1].coefficient());
        assert_eq!("bonus", f.exp().rhs()[2].name());
        assert_eq!(1000.0, f.exp().rhs()[2].value());
    }

    #[test]
    fn can_create_constraints() {
        let e: Expression = Expression::new(vec![new_var("x", 2.0), new_var("y", 3.0)],
                                            Relationship::LEQ,
                                            vec![new_const("volume", 2300.0)]);
        let c1: Constraint = Constraint::Regular(RefCell::new(e));
        match c1 {
            Constraint::Regular(ref_cell) => {
                let exp = ref_cell.borrow();
                assert_eq!("x", exp.lhs()[0].name());
                assert_eq!(2.0, exp.lhs()[0].coefficient());
                assert_eq!(Relationship::LEQ, *exp.rel());
                assert_eq!("y", exp.lhs()[1].name());
                assert_eq!(3.0, exp.lhs()[1].coefficient());
                assert_eq!("volume", exp.rhs()[0].name());
                assert_eq!(2300.0, exp.rhs()[0].value());
            }
            _ => panic!("Unexpected variant."),
        }
        let c2: Constraint = Constraint::NonNegative(new_var("x", 2.0));
        match c2 {
            Constraint::NonNegative(abst_var) => {
                assert_eq!("x", abst_var.name());
                assert_eq!(2.0, abst_var.coefficient());
            }
            _ => panic!("Unexpected variant."),
        }
    }

    #[test]
    fn can_create_system_of_constraints() {
        let e: Expression = Expression::new(vec![new_var("x", 2.0), new_var("y", 3.0)],
                                            Relationship::LEQ,
                                            vec![new_const("volume", 2300.0)]);
        let c1: Constraint = Constraint::Regular(RefCell::new(e));
        let c2: Constraint = Constraint::NonNegative(new_var("x", 2.0));
        let s: SystemOfConstraints = SystemOfConstraints::new(vec![c1, c2]);
        for constraint in s.system() {
            match constraint {
                &Constraint::Regular(ref ref_cell) => {
                    let exp = ref_cell.borrow();
                    assert_eq!("x", exp.lhs()[0].name());
                    assert_eq!(2.0, exp.lhs()[0].coefficient());
                    assert_eq!(Relationship::LEQ, *exp.rel());
                    assert_eq!("y", exp.lhs()[1].name());
                    assert_eq!(3.0, exp.lhs()[1].coefficient());
                    assert_eq!("volume", exp.rhs()[0].name());
                    assert_eq!(2300.0, exp.rhs()[0].value());
                }
                &Constraint::NonNegative(ref abst_var) => {
                    assert_eq!("x", abst_var.name());
                    assert_eq!(2.0, abst_var.coefficient());
                }
            }
        }
    }

    #[test]
    fn can_transform_geq_rels() {
        let e1: Expression = Expression::new(vec![new_var("x", 2.0), new_var("y", 3.0)],
                                             Relationship::LEQ,
                                             vec![new_const("volume", 2300.0)]);
        let e2: Expression = Expression::new(vec![new_var("w", 6.0), new_var("z", 9.0)],
                                             Relationship::GEQ,
                                             vec![new_const("area", 300.0)]);
        let c1: Constraint = Constraint::Regular(RefCell::new(e1));
        let c2: Constraint = Constraint::Regular(RefCell::new(e2));
        let c3: Constraint = Constraint::NonNegative(new_var("x", 2.0));
        let s: SystemOfConstraints = SystemOfConstraints::new(vec![c1, c2, c3]);
        transform_geq_rels(&s);
        match s.system()[0] {
            Constraint::Regular(ref ref_cell) => {
                let exp = ref_cell.borrow();
                assert_eq!("x", exp.lhs()[0].name());
                assert_eq!(2.0, exp.lhs()[0].coefficient());
                assert_eq!(Relationship::LEQ, *exp.rel());
                assert_eq!("y", exp.lhs()[1].name());
                assert_eq!(3.0, exp.lhs()[1].coefficient());
                assert_eq!("volume", exp.rhs()[0].name());
                assert_eq!(2300.0, exp.rhs()[0].value());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
        match s.system()[1] {
            Constraint::Regular(ref ref_cell) => {
                let exp = ref_cell.borrow();
                assert_eq!("w", exp.lhs()[0].name());
                assert_eq!(-6.0, exp.lhs()[0].coefficient());
                assert_eq!(Relationship::LEQ, *exp.rel());
                assert_eq!("z", exp.lhs()[1].name());
                assert_eq!(-9.0, exp.lhs()[1].coefficient());
                assert_eq!("area", exp.rhs()[0].name());
                assert_eq!(-300.0, exp.rhs()[0].value());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
        match s.system()[2] {
            Constraint::NonNegative(ref abst_var) => {
                assert_eq!("x", abst_var.name());
                assert_eq!(2.0, abst_var.coefficient());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
    }

    #[test]
    fn can_transform_leq_rels() {
        let e1: Expression = Expression::new(vec![new_var("x", 2.0), new_var("y", 3.0)],
                                             Relationship::LEQ,
                                             vec![new_const("volume", 2300.0)]);
        let e2: Expression = Expression::new(vec![new_var("w", 6.0), new_var("z", 9.0)],
                                             Relationship::LEQ,
                                             vec![new_const("area", 300.0)]);
        let c1: Constraint = Constraint::Regular(RefCell::new(e1));
        let c2: Constraint = Constraint::Regular(RefCell::new(e2));
        let c3: Constraint = Constraint::NonNegative(new_var("x", 2.0));
        let s: SystemOfConstraints = SystemOfConstraints::new(vec![c1, c2, c3]);
        transform_leq_rels(&s);
        match s.system()[0] {
            Constraint::Regular(ref ref_cell) => {
                let exp = ref_cell.borrow();
                assert_eq!("x", exp.lhs()[0].name());
                assert_eq!(2.0, exp.lhs()[0].coefficient());
                assert_eq!(Relationship::EQ, *exp.rel());
                assert_eq!("y", exp.lhs()[1].name());
                assert_eq!(3.0, exp.lhs()[1].coefficient());
                assert_eq!("s1", exp.lhs()[2].name());
                assert_eq!("volume", exp.rhs()[0].name());
                assert_eq!(2300.0, exp.rhs()[0].value());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
        match s.system()[1] {
            Constraint::Regular(ref ref_cell) => {
                let exp = ref_cell.borrow();
                assert_eq!("w", exp.lhs()[0].name());
                assert_eq!(6.0, exp.lhs()[0].coefficient());
                assert_eq!(Relationship::EQ, *exp.rel());
                assert_eq!("z", exp.lhs()[1].name());
                assert_eq!(9.0, exp.lhs()[1].coefficient());
                assert_eq!("s2", exp.lhs()[2].name());
                assert_eq!("area", exp.rhs()[0].name());
                assert_eq!(300.0, exp.rhs()[0].value());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
        match s.system()[2] {
            Constraint::NonNegative(ref abst_var) => {
                assert_eq!("x", abst_var.name());
                assert_eq!(2.0, abst_var.coefficient());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
    }
}
