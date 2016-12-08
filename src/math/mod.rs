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
        assert_eq!(2.0, v.coefficient());
    }

    #[test]
    fn can_set_coefficient() {
        let abst_var_state: RefCell<AbstVar> = RefCell::new(new_var("x", 2.0));
        {
            let mut abst_var = abst_var_state.borrow_mut();
            abst_var.set_coefficient(3.0);
        }
        assert_eq!(3.0, abst_var_state.borrow().coefficient());
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
        assert_eq!(450.0, c.value());
    }

    #[test]
    fn can_set_value() {
        let abst_var_state: RefCell<AbstVar> = RefCell::new(new_const("days", 365.0));
        {
            let mut abst_var = abst_var_state.borrow_mut();
            abst_var.set_value(366.0);
        }
        assert_eq!(366.0, abst_var_state.borrow().value());
    }

    #[test]
    fn can_create_slack_var() {
        let s_var: AbstVar = new_slack_var("s1".to_string());
        assert_eq!("s1", s_var.name());
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
        assert_eq!(1.0, e.lhs()[0].coefficient());
        assert_eq!(Relationship::EQ, *e.rel());
        assert_eq!("x", e.rhs()[0].name());
        assert_eq!(2.0, e.rhs()[0].coefficient());
        assert_eq!("y", e.rhs()[1].name());
        assert_eq!(3.0, e.rhs()[1].coefficient());
        assert_eq!("bonus", e.rhs()[2].name());
        assert_eq!(1000.0, e.rhs()[2].value());
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
        assert_eq!(1.0, exp.lhs()[0].coefficient());
        assert_eq!("w", exp.lhs()[1].name());
        assert_eq!(9.0, exp.lhs()[1].coefficient());
        assert_eq!("weight", exp.lhs()[2].name());
        assert_eq!(700.0, exp.lhs()[2].value());
        assert_eq!("s1", exp.lhs()[3].name());
        assert_eq!(Relationship::EQ, *exp.rel());
        assert_eq!("x", exp.rhs()[0].name());
        assert_eq!(2.0, exp.rhs()[0].coefficient());
        assert_eq!("y", exp.rhs()[1].name());
        assert_eq!(3.0, exp.rhs()[1].coefficient());
        assert_eq!("bonus", exp.rhs()[2].name());
        assert_eq!(1000.0, exp.rhs()[2].value());
    }

    #[test]
    fn can_mul_both_sides_of_expressions() {
        let mut e1: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::GEQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        e1.mul_both_sides(-1.0);
        assert_eq!("Z", e1.lhs()[0].name());
        assert_eq!(-1.0, e1.lhs()[0].coefficient());
        assert_eq!(Relationship::LEQ, *e1.rel());
        assert_eq!("x", e1.rhs()[0].name());
        assert_eq!(-2.0, e1.rhs()[0].coefficient());
        assert_eq!("y", e1.rhs()[1].name());
        assert_eq!(-3.0, e1.rhs()[1].coefficient());
        assert_eq!("bonus", e1.rhs()[2].name());
        assert_eq!(-1000.0, e1.rhs()[2].value());
    }
}
