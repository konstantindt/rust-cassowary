pub mod variables;
pub mod relationships;
pub mod expressions;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use math::variables::{AbstVar, new_var, new_const, new_slack_var};
    use math::relationships::Relationship;
    use math::expressions::Expression;

    #[test]
    fn can_create_variables() {
        let v: AbstVar = new_var("x", 2.0);
        assert_eq!(AbstVar::Variable {
                       name: "x".to_string(),
                       coefficient: 2.0,
                   },
                   v);
        assert_eq!("x", v.name());
        assert_eq!(2.0, v.get_data());
    }

    #[test]
    fn can_set_coefficient() {
        let abst_var_state: RefCell<AbstVar> = RefCell::new(new_var("x", 2.0));
        {
            let mut abst_var = abst_var_state.borrow_mut();
            abst_var.set_data(3.0);
        }
        assert_eq!(3.0, abst_var_state.borrow().get_data());
    }

    #[test]
    fn can_create_constants() {
        let c: AbstVar = new_const("barrels in stock", 450.0);
        assert_eq!(AbstVar::Constant {
                       name: "barrels in stock".to_string(),
                       value: 450.0,
                   },
                   c);
        assert_eq!("barrels in stock", c.name());
        assert_eq!(450.0, c.get_data());
    }

    #[test]
    fn can_set_value() {
        let abst_var_state: RefCell<AbstVar> = RefCell::new(new_const("days", 365.0));
        {
            let mut abst_var = abst_var_state.borrow_mut();
            abst_var.set_data(366.0);
        }
        assert_eq!(366.0, abst_var_state.borrow().get_data());
    }

    #[test]
    fn can_create_slack_var() {
        let s_var: AbstVar = new_slack_var("s1".to_string());
        assert_eq!("s1", s_var.name());
    }

    #[test]
    fn can_get_from_slack_var() {
        let s_var: AbstVar = new_slack_var("s1".to_string());
        assert_eq!(1.0, s_var.get_data());
    }

    #[test]
    fn can_create_relationships() {
        let r: Relationship = Relationship::EQ;
        assert_eq!(Relationship::EQ, r);
        let r: Relationship = Relationship::LEQ;
        assert_eq!(Relationship::LEQ, r);
        let r: Relationship = Relationship::GEQ;
        assert_eq!(Relationship::GEQ, r);
    }

    #[test]
    fn can_create_expressions() {
        let e: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        assert_eq!("Z", e.lhs()[0].name());
        assert_eq!(1.0, e.lhs()[0].get_data());
        assert_eq!(Relationship::EQ, *e.rel());
        assert_eq!("x", e.rhs()[0].name());
        assert_eq!(2.0, e.rhs()[0].get_data());
        assert_eq!("y", e.rhs()[1].name());
        assert_eq!(3.0, e.rhs()[1].get_data());
        assert_eq!("bonus", e.rhs()[2].name());
        assert_eq!(1000.0, e.rhs()[2].get_data());
    }

    #[test]
    fn can_set_rels() {
        let e: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::LEQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        let e_state: RefCell<Expression> = RefCell::new(e);
        {
            let mut exp = e_state.borrow_mut();
            exp.set_rel(Relationship::EQ);
        }
        let exp = e_state.borrow();
        assert_eq!(Relationship::EQ, *exp.rel());
    }

    #[test]
    fn can_add_lhs() {
        let e: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        let e_state: RefCell<Expression> = RefCell::new(e);
        {
            let mut exp = e_state.borrow_mut();
            exp.add_lhs(new_var("w", 9.0));
            exp.add_lhs(new_const("weight", 700.0));
            exp.add_lhs(new_slack_var("s1".to_string()));
        }
        let exp = e_state.borrow();
        assert_eq!("Z", exp.lhs()[0].name());
        assert_eq!(1.0, exp.lhs()[0].get_data());
        assert_eq!("w", exp.lhs()[1].name());
        assert_eq!(9.0, exp.lhs()[1].get_data());
        assert_eq!("weight", exp.lhs()[2].name());
        assert_eq!(700.0, exp.lhs()[2].get_data());
        assert_eq!("s1", exp.lhs()[3].name());
        assert_eq!(Relationship::EQ, *exp.rel());
        assert_eq!("x", exp.rhs()[0].name());
        assert_eq!(2.0, exp.rhs()[0].get_data());
        assert_eq!("y", exp.rhs()[1].name());
        assert_eq!(3.0, exp.rhs()[1].get_data());
        assert_eq!("bonus", exp.rhs()[2].name());
        assert_eq!(1000.0, exp.rhs()[2].get_data());
    }

    #[test]
    fn can_move_from_lhs_side() {
        let mut e1: Expression =
            Expression::new(vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)],
                            Relationship::GEQ,
                            vec![new_var("Z", 1.0)]);

        e1.move_from_lhs_side(1, false);
        assert_eq!(2, e1.lhs().len());
        assert_eq!("x", e1.lhs()[0].name());
        assert_eq!(2.0, e1.lhs()[0].get_data());
        assert_eq!("bonus", e1.lhs()[1].name());
        assert_eq!(1000.0, e1.lhs()[1].get_data());
        assert_eq!(Relationship::GEQ, *e1.rel());
        assert_eq!(2, e1.rhs().len());
        assert_eq!("Z", e1.rhs()[0].name());
        assert_eq!(1.0, e1.rhs()[0].get_data());
        assert_eq!("y", e1.rhs()[1].name());
        assert_eq!(-3.0, e1.rhs()[1].get_data());

        e1.move_from_lhs_side(0, true);
        assert_eq!(1, e1.lhs().len());
        assert_eq!("bonus", e1.lhs()[0].name());
        assert_eq!(1000.0, e1.lhs()[0].get_data());
        assert_eq!(Relationship::GEQ, *e1.rel());
        assert_eq!(3, e1.rhs().len());
        assert_eq!("x", e1.rhs()[0].name());
        assert_eq!(-2.0, e1.rhs()[0].get_data());
        assert_eq!("Z", e1.rhs()[1].name());
        assert_eq!(1.0, e1.rhs()[1].get_data());
        assert_eq!("y", e1.rhs()[2].name());
        assert_eq!(-3.0, e1.rhs()[2].get_data());

        e1.move_from_lhs_side(0, false);
        assert_eq!(1, e1.lhs().len());
        assert_eq!("zero", e1.lhs()[0].name());
        assert_eq!(0.0, e1.lhs()[0].get_data());
        assert_eq!(Relationship::GEQ, *e1.rel());
        assert_eq!(4, e1.rhs().len());
        assert_eq!("x", e1.rhs()[0].name());
        assert_eq!(-2.0, e1.rhs()[0].get_data());
        assert_eq!("Z", e1.rhs()[1].name());
        assert_eq!(1.0, e1.rhs()[1].get_data());
        assert_eq!("y", e1.rhs()[2].name());
        assert_eq!(-3.0, e1.rhs()[2].get_data());
        assert_eq!("bonus", e1.rhs()[3].name());
        assert_eq!(-1000.0, e1.rhs()[3].get_data());


        let mut e2: Expression =
            Expression::new(vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)],
                            Relationship::GEQ,
                            vec![new_var("Z", 1.0), new_slack_var("s1".to_string())]);

        e2.move_from_lhs_side(1, false);
        assert_eq!(2, e2.lhs().len());
        assert_eq!("x", e2.lhs()[0].name());
        assert_eq!(2.0, e2.lhs()[0].get_data());
        assert_eq!("bonus", e2.lhs()[1].name());
        assert_eq!(1000.0, e2.lhs()[1].get_data());
        assert_eq!(Relationship::GEQ, *e2.rel());
        assert_eq!(3, e2.rhs().len());
        assert_eq!("Z", e2.rhs()[0].name());
        assert_eq!(1.0, e2.rhs()[0].get_data());
        assert_eq!("y", e2.rhs()[1].name());
        assert_eq!(-3.0, e2.rhs()[1].get_data());
        assert_eq!("s1", e2.rhs()[2].name());
        assert_eq!(1.0, e2.rhs()[2].get_data());

        e2.move_from_lhs_side(0, true);
        assert_eq!(1, e2.lhs().len());
        assert_eq!("bonus", e2.lhs()[0].name());
        assert_eq!(1000.0, e2.lhs()[0].get_data());
        assert_eq!(Relationship::GEQ, *e2.rel());
        assert_eq!(4, e2.rhs().len());
        assert_eq!("x", e2.rhs()[0].name());
        assert_eq!(-2.0, e2.rhs()[0].get_data());
        assert_eq!("Z", e2.rhs()[1].name());
        assert_eq!(1.0, e2.rhs()[1].get_data());
        assert_eq!("y", e2.rhs()[2].name());
        assert_eq!(-3.0, e2.rhs()[2].get_data());
        assert_eq!("s1", e2.rhs()[3].name());
        assert_eq!(1.0, e2.rhs()[3].get_data());

        e2.move_from_lhs_side(0, true);
        assert_eq!(1, e2.lhs().len());
        assert_eq!("zero", e2.lhs()[0].name());
        assert_eq!(0.0, e2.lhs()[0].get_data());
        assert_eq!(Relationship::GEQ, *e2.rel());
        assert_eq!(5, e2.rhs().len());
        assert_eq!("bonus", e2.rhs()[0].name());
        assert_eq!(-1000.0, e2.rhs()[0].get_data());
        assert_eq!("x", e2.rhs()[1].name());
        assert_eq!(-2.0, e2.rhs()[1].get_data());
        assert_eq!("Z", e2.rhs()[2].name());
        assert_eq!(1.0, e2.rhs()[2].get_data());
        assert_eq!("y", e2.rhs()[3].name());
        assert_eq!(-3.0, e2.rhs()[3].get_data());
        assert_eq!("s1", e2.rhs()[4].name());
        assert_eq!(1.0, e2.rhs()[4].get_data());
    }

    #[test]
    fn can_mul_both_sides_of_expressions() {
        let mut e1: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::GEQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        e1.mul_both_sides(-1.0);
        assert_eq!("Z", e1.lhs()[0].name());
        assert_eq!(-1.0, e1.lhs()[0].get_data());
        assert_eq!(Relationship::LEQ, *e1.rel());
        assert_eq!("x", e1.rhs()[0].name());
        assert_eq!(-2.0, e1.rhs()[0].get_data());
        assert_eq!("y", e1.rhs()[1].name());
        assert_eq!(-3.0, e1.rhs()[1].get_data());
        assert_eq!("bonus", e1.rhs()[2].name());
        assert_eq!(-1000.0, e1.rhs()[2].get_data());
    }

    #[test]
    fn can_swap_sides() {
        let mut exp: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);

        exp.swap_sides().unwrap();
        assert_eq!(3, exp.lhs().len());
        assert_eq!("x", exp.lhs()[0].name());
        assert_eq!(2.0, exp.lhs()[0].get_data());
        assert_eq!("y", exp.lhs()[1].name());
        assert_eq!(3.0, exp.lhs()[1].get_data());
        assert_eq!("bonus", exp.lhs()[2].name());
        assert_eq!(1000.0, exp.lhs()[2].get_data());
        assert_eq!(Relationship::EQ, *exp.rel());
        assert_eq!(1, exp.rhs().len());
        assert_eq!("Z", exp.rhs()[0].name());
        assert_eq!(1.0, exp.rhs()[0].get_data());

        exp.set_rel(Relationship::LEQ);
        assert!(exp.swap_sides().is_err());

        exp.set_rel(Relationship::GEQ);
        assert!(exp.swap_sides().is_err());
    }
}
