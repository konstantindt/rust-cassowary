pub mod problems;
pub mod functions;
pub mod constraints;
pub mod solvers;

#[cfg(test)]
mod tests {
    use math::variables::{AbstVar, new_var, new_const};
    use math::relationships::Relationship;
    use math::expressions::Expression;
    use objective::problems::ProblemType;
    use objective::functions::Function;
    use objective::constraints::{Constraint, new_reg_con, new_non_neg_con, SystemOfConstraints};
    use objective::solvers::{transform_constraint_rels_to_eq, rearrange_fun_eq_zero};

    #[test]
    fn can_create_problem_types() {
        let p_t_max = ProblemType::MAX;
        assert_eq!(ProblemType::MAX, p_t_max);
        let p_t_min = ProblemType::MIN;
        assert_eq!(ProblemType::MIN, p_t_min);
    }

    #[test]
    fn can_create_functions() {
        let e1 =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        let f1 = Function::new(e1, ProblemType::MAX);
        let exp1 = f1.exp().borrow();
        assert_eq!("Z", f1.name());
        assert_eq!(ProblemType::MAX, *f1.p_type());
        assert_eq!("Z", exp1.lhs()[0].name());
        assert_eq!(1.0, exp1.lhs()[0].get_data());
        assert_eq!(Relationship::EQ, *exp1.rel());
        assert_eq!("x", exp1.rhs()[0].name());
        assert_eq!(2.0, exp1.rhs()[0].get_data());
        assert_eq!("y", exp1.rhs()[1].name());
        assert_eq!(3.0, exp1.rhs()[1].get_data());
        assert_eq!("bonus", exp1.rhs()[2].name());
        assert_eq!(1000.0, exp1.rhs()[2].get_data());

        let exp1_max = f1.exp_max().borrow();
        assert_eq!("Z", exp1_max.lhs()[0].name());
        assert_eq!(1.0, exp1_max.lhs()[0].get_data());
        assert_eq!(Relationship::EQ, *exp1_max.rel());
        assert_eq!("x", exp1_max.rhs()[0].name());
        assert_eq!(2.0, exp1_max.rhs()[0].get_data());
        assert_eq!("y", exp1_max.rhs()[1].name());
        assert_eq!(3.0, exp1_max.rhs()[1].get_data());
        assert_eq!("bonus", exp1_max.rhs()[2].name());
        assert_eq!(1000.0, exp1_max.rhs()[2].get_data());

        let e2 =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        let f2 = Function::new(e2, ProblemType::MIN);
        let exp2 = f2.exp().borrow();
        assert_eq!("Z", f2.name());
        assert_eq!(ProblemType::MIN, *f2.p_type());
        assert_eq!("Z", exp2.lhs()[0].name());
        assert_eq!(1.0, exp2.lhs()[0].get_data());
        assert_eq!(Relationship::EQ, *exp2.rel());
        assert_eq!("x", exp2.rhs()[0].name());
        assert_eq!(2.0, exp2.rhs()[0].get_data());
        assert_eq!("y", exp2.rhs()[1].name());
        assert_eq!(3.0, exp2.rhs()[1].get_data());
        assert_eq!("bonus", exp2.rhs()[2].name());
        assert_eq!(1000.0, exp2.rhs()[2].get_data());

        let exp2_max = f2.exp_max().borrow();
        assert_eq!("Q", exp2_max.lhs()[0].name());
        assert_eq!(1.0, exp2_max.lhs()[0].get_data());
        assert_eq!(Relationship::EQ, *exp2_max.rel());
        assert_eq!("x", exp2_max.rhs()[0].name());
        assert_eq!(-2.0, exp2_max.rhs()[0].get_data());
        assert_eq!("y", exp2_max.rhs()[1].name());
        assert_eq!(-3.0, exp2_max.rhs()[1].get_data());
        assert_eq!("bonus", exp2_max.rhs()[2].name());
        assert_eq!(-1000.0, exp2_max.rhs()[2].get_data());
    }

    #[test]
    fn can_create_constraints() {
        let exp = Expression::new(vec![new_var("x", 2.0), new_var("y", 3.0)],
                                  Relationship::LEQ,
                                  vec![new_const("volume", 2300.0)]);
        let c1 = new_reg_con(exp);
        match c1 {
            Constraint::Regular(ref_cell) => {
                let exp = ref_cell.borrow();
                assert_eq!("x", exp.lhs()[0].name());
                assert_eq!(2.0, exp.lhs()[0].get_data());
                assert_eq!(Relationship::LEQ, *exp.rel());
                assert_eq!("y", exp.lhs()[1].name());
                assert_eq!(3.0, exp.lhs()[1].get_data());
                assert_eq!("volume", exp.rhs()[0].name());
                assert_eq!(2300.0, exp.rhs()[0].get_data());
            }
            _ => panic!("Unexpected variant."),
        }

        let c2 = new_non_neg_con(new_var("x", 2.0));
        match c2 {
            Constraint::NonNegative(abst_var) => {
                assert_eq!("x", abst_var.name());
                assert_eq!(2.0, abst_var.get_data());
            }
            _ => panic!("Unexpected variant."),
        }
    }

    #[test]
    fn can_create_system_of_constraints() {
        let exp = Expression::new(vec![new_var("x", 2.0), new_var("y", 3.0)],
                                  Relationship::LEQ,
                                  vec![new_const("volume", 2300.0)]);
        let c1 = new_reg_con(exp);
        let c2 = new_non_neg_con(new_var("x", 2.0));
        let s = SystemOfConstraints::new(vec![c1, c2]);
        for constraint in s.system() {
            match constraint {
                &Constraint::Regular(ref ref_cell) => {
                    let exp = ref_cell.borrow();
                    assert_eq!("x", exp.lhs()[0].name());
                    assert_eq!(2.0, exp.lhs()[0].get_data());
                    assert_eq!(Relationship::LEQ, *exp.rel());
                    assert_eq!("y", exp.lhs()[1].name());
                    assert_eq!(3.0, exp.lhs()[1].get_data());
                    assert_eq!("volume", exp.rhs()[0].name());
                    assert_eq!(2300.0, exp.rhs()[0].get_data());
                }
                &Constraint::NonNegative(ref abst_var) => {
                    assert_eq!("x", abst_var.name());
                    assert_eq!(2.0, abst_var.get_data());
                }
            }
        }
    }

    #[test]
    fn can_transform_constraint_rels_to_eq() {
        let exp1 = Expression::new(vec![new_var("x", 2.0), new_var("y", 3.0)],
                                   Relationship::LEQ,
                                   vec![new_const("volume", 2300.0)]);
        let exp2 = Expression::new(vec![new_var("w", 6.0), new_var("z", 9.0)],
                                   Relationship::GEQ,
                                   vec![new_const("area", 300.0)]);
        let exp3 = Expression::new(vec![new_var("u", 61.0), new_var("t", 19.0)],
                                   Relationship::GEQ,
                                   vec![new_const("hyperplane", -3000.0)]);
        let c1 = new_reg_con(exp1);
        let c2 = new_reg_con(exp2);
        let c3 = new_reg_con(exp3);
        let c4 = new_non_neg_con(new_var("x", 2.0));
        let s = SystemOfConstraints::new(vec![c1, c2, c3, c4]);
        transform_constraint_rels_to_eq(&s);
        match s.system()[0] {
            Constraint::Regular(ref ref_cell) => {
                let exp = ref_cell.borrow();
                assert_eq!("x", exp.lhs()[0].name());
                assert_eq!(2.0, exp.lhs()[0].get_data());
                assert_eq!(Relationship::EQ, *exp.rel());
                assert_eq!("y", exp.lhs()[1].name());
                assert_eq!(3.0, exp.lhs()[1].get_data());
                assert_eq!(AbstVar::SlackVar { name: "sl1".to_string() }, exp.lhs()[2]);
                assert_eq!("volume", exp.rhs()[0].name());
                assert_eq!(2300.0, exp.rhs()[0].get_data());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
        match s.system()[1] {
            Constraint::Regular(ref ref_cell) => {
                let exp = ref_cell.borrow();
                assert_eq!("w", exp.lhs()[0].name());
                assert_eq!(6.0, exp.lhs()[0].get_data());
                assert_eq!(Relationship::EQ, *exp.rel());
                assert_eq!("z", exp.lhs()[1].name());
                assert_eq!(9.0, exp.lhs()[1].get_data());
                assert_eq!(AbstVar::SurplusVar { name: "su2".to_string() },
                           exp.lhs()[2]);
                assert_eq!("area", exp.rhs()[0].name());
                assert_eq!(300.0, exp.rhs()[0].get_data());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
        match s.system()[2] {
            Constraint::Regular(ref ref_cell) => {
                let exp = ref_cell.borrow();
                assert_eq!("u", exp.lhs()[0].name());
                assert_eq!(-61.0, exp.lhs()[0].get_data());
                assert_eq!(Relationship::EQ, *exp.rel());
                assert_eq!("t", exp.lhs()[1].name());
                assert_eq!(-19.0, exp.lhs()[1].get_data());
                assert_eq!(AbstVar::SlackVar { name: "sl3".to_string() }, exp.lhs()[2]);
                assert_eq!("hyperplane", exp.rhs()[0].name());
                assert_eq!(3000.0, exp.rhs()[0].get_data());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
        match s.system()[3] {
            Constraint::NonNegative(ref abst_var) => {
                assert_eq!("x", abst_var.name());
                assert_eq!(2.0, abst_var.get_data());
            }
            _ => panic!("Unexpected variant in this program logic."),
        };
    }

    #[test]
    fn can_rearrange_fun_eq_zero() {
        let e1 =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        let mut f1 = Function::new(e1, ProblemType::MAX);
        rearrange_fun_eq_zero(&mut f1);
        let exp1 = f1.exp().borrow();
        assert_eq!(ProblemType::MAX, *f1.p_type());
        assert_eq!("zero", exp1.rhs()[0].name());
        assert_eq!(0.0, exp1.rhs()[0].get_data());
        assert_eq!(Relationship::EQ, *exp1.rel());
        assert_eq!("x", exp1.lhs()[0].name());
        assert_eq!(-2.0, exp1.lhs()[0].get_data());
        assert_eq!("y", exp1.lhs()[1].name());
        assert_eq!(-3.0, exp1.lhs()[1].get_data());
        assert_eq!("bonus", exp1.lhs()[2].name());
        assert_eq!(-1000.0, exp1.lhs()[2].get_data());
        assert_eq!("Z", exp1.lhs()[3].name());
        assert_eq!(1.0, exp1.lhs()[3].get_data());

        let exp1_max = f1.exp_max().borrow();
        assert_eq!("zero", exp1_max.rhs()[0].name());
        assert_eq!(0.0, exp1_max.rhs()[0].get_data());
        assert_eq!(Relationship::EQ, *exp1_max.rel());
        assert_eq!("x", exp1_max.lhs()[0].name());
        assert_eq!(-2.0, exp1_max.lhs()[0].get_data());
        assert_eq!("y", exp1_max.lhs()[1].name());
        assert_eq!(-3.0, exp1_max.lhs()[1].get_data());
        assert_eq!("bonus", exp1_max.lhs()[2].name());
        assert_eq!(-1000.0, exp1_max.lhs()[2].get_data());
        assert_eq!("Z", exp1_max.lhs()[3].name());
        assert_eq!(1.0, exp1_max.lhs()[3].get_data());

        let e2 =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        let mut f2 = Function::new(e2, ProblemType::MIN);
        rearrange_fun_eq_zero(&mut f2);
        let exp2 = f2.exp().borrow();
        assert_eq!(ProblemType::MIN, *f2.p_type());
        assert_eq!("Z", exp2.lhs()[0].name());
        assert_eq!(1.0, exp2.lhs()[0].get_data());
        assert_eq!(Relationship::EQ, *exp2.rel());
        assert_eq!("x", exp2.rhs()[0].name());
        assert_eq!(2.0, exp2.rhs()[0].get_data());
        assert_eq!("y", exp2.rhs()[1].name());
        assert_eq!(3.0, exp2.rhs()[1].get_data());
        assert_eq!("bonus", exp2.rhs()[2].name());
        assert_eq!(1000.0, exp2.rhs()[2].get_data());

        let exp2_max = f2.exp_max().borrow();
        assert_eq!("zero", exp2_max.rhs()[0].name());
        assert_eq!(0.0, exp2_max.rhs()[0].get_data());
        assert_eq!(Relationship::EQ, *exp2_max.rel());
        assert_eq!("x", exp2_max.lhs()[0].name());
        assert_eq!(2.0, exp2_max.lhs()[0].get_data());
        assert_eq!("y", exp2_max.lhs()[1].name());
        assert_eq!(3.0, exp2_max.lhs()[1].get_data());
        assert_eq!("bonus", exp2_max.lhs()[2].name());
        assert_eq!(1000.0, exp2_max.lhs()[2].get_data());
        assert_eq!("Q", exp2_max.lhs()[3].name());
        assert_eq!(1.0, exp2_max.lhs()[3].get_data());
    }
}
