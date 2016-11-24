pub mod variables;
pub mod relationships;
pub mod expressions;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use math::variables::{AbstVar, new_var, new_const};
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
        let v_state: RefCell<AbstVar> = RefCell::new(new_var("x", 2.0));
        {
            let mut v = v_state.borrow_mut();
            v.set_coefficient(3.0);
        }
        assert_eq!(3.0, v_state.borrow().coefficient());
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
}
