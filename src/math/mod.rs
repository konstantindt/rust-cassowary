pub mod variables;
pub mod relationships;
pub mod expressions;

#[cfg(test)]
mod tests {
    use math::variables::{AbstVar, new_var, new_const, new_slack_var, new_surplus_var, new_arti_var};
    use math::relationships::Relationship;
    use math::expressions::Expression;

    #[test]
    fn can_create_variables() {
        let v1 = new_var("x", 2.0);
        assert_eq!(AbstVar::Variable {
                       name: "x".to_string(),
                       coefficient: 2.0,
                   },
                   v1);
        assert_eq!("x", v1.name());
        assert_eq!(2.0, v1.get_data());

        let c1 = new_const("barrels in stock", 450.0);
        assert_eq!(AbstVar::Constant {
                       name: "barrels in stock".to_string(),
                       value: 450.0,
                   },
                   c1);
        assert_eq!("barrels in stock", c1.name());
        assert_eq!(450.0, c1.get_data());

        let sl_var = new_slack_var("sl1".to_string());
        assert_eq!(AbstVar::SlackVar { name: "sl1".to_string() }, sl_var);
        assert_eq!("sl1", sl_var.name());
        assert_eq!(1.0, sl_var.get_data());

        let su_var = new_surplus_var("su1".to_string());
        assert_eq!(AbstVar::SurplusVar { name: "su1".to_string() }, su_var);
        assert_eq!("su1", su_var.name());
        assert_eq!(-1.0, su_var.get_data());

        let arti_var = new_arti_var("arti1".to_string());
        assert_eq!(AbstVar::ArtiVar { name: "arti1".to_string() }, arti_var);
        assert_eq!("arti1", arti_var.name());
        assert_eq!(1.0, arti_var.get_data());
    }

    #[test]
    fn can_set_data() {
        let mut c2 = new_const("barrels in stock", 450.0);
        c2.set_data(366.0);
        assert_eq!("barrels in stock", c2.name());
        assert_eq!(366.0, c2.get_data());

        let mut v2 = new_var("x", 2.0);
        v2.set_data(3.0);
        assert_eq!(3.0, v2.get_data());
        assert_eq!("x", v2.name());
    }

    #[test]
    fn can_create_relationships() {
        let r1: Relationship = Relationship::EQ;
        assert_eq!(Relationship::EQ, r1);
        let r2: Relationship = Relationship::LEQ;
        assert_eq!(Relationship::LEQ, r2);
        let r3: Relationship = Relationship::GEQ;
        assert_eq!(Relationship::GEQ, r3);
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
        let mut exp: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::LEQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        exp.set_rel(Relationship::EQ);
        assert_eq!(Relationship::EQ, *exp.rel());
    }

    #[test]
    fn can_add_to_sides() {
        let mut exp1: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        exp1.add_lhs(new_var("w", 9.0));
        exp1.add_lhs(new_const("weight", 700.0));
        exp1.add_lhs(new_slack_var("s1".to_string()));
        exp1.add_lhs(new_var("Z", 2.0));
        assert_eq!("Z", exp1.lhs()[0].name());
        assert_eq!(3.0, exp1.lhs()[0].get_data());
        assert_eq!("w", exp1.lhs()[1].name());
        assert_eq!(9.0, exp1.lhs()[1].get_data());
        assert_eq!("weight", exp1.lhs()[2].name());
        assert_eq!(700.0, exp1.lhs()[2].get_data());
        assert_eq!("s1", exp1.lhs()[3].name());
        assert_eq!(Relationship::EQ, *exp1.rel());
        assert_eq!("x", exp1.rhs()[0].name());
        assert_eq!(2.0, exp1.rhs()[0].get_data());
        assert_eq!("y", exp1.rhs()[1].name());
        assert_eq!(3.0, exp1.rhs()[1].get_data());
        assert_eq!("bonus", exp1.rhs()[2].name());
        assert_eq!(1000.0, exp1.rhs()[2].get_data());

        let mut exp2: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::EQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        exp2.add_rhs(new_var("w", 9.0));
        exp2.add_rhs(new_const("weight", 700.0));
        exp2.add_rhs(new_slack_var("s1".to_string()));
        exp2.add_rhs(new_const("bonus", 500.0));
        assert_eq!("Z", exp2.lhs()[0].name());
        assert_eq!(1.0, exp2.lhs()[0].get_data());
        assert_eq!(Relationship::EQ, *exp2.rel());
        assert_eq!("x", exp2.rhs()[0].name());
        assert_eq!(2.0, exp2.rhs()[0].get_data());
        assert_eq!("y", exp2.rhs()[1].name());
        assert_eq!(3.0, exp2.rhs()[1].get_data());
        assert_eq!("bonus", exp2.rhs()[2].name());
        assert_eq!(1500.0, exp2.rhs()[2].get_data());
        assert_eq!("w", exp2.rhs()[3].name());
        assert_eq!(9.0, exp2.rhs()[3].get_data());
        assert_eq!("weight", exp2.rhs()[4].name());
        assert_eq!(700.0, exp2.rhs()[4].get_data());
        assert_eq!("s1", exp2.rhs()[5].name());
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
        assert_eq!("RHS", e1.lhs()[0].name());
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
        assert_eq!("RHS", e2.lhs()[0].name());
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
        let mut exp: Expression =
            Expression::new(vec![new_var("Z", 1.0)],
                            Relationship::GEQ,
                            vec![new_var("x", 2.0), new_var("y", 3.0), new_const("bonus", 1000.0)]);
        exp.mul_both_sides(-1.0);
        assert_eq!("Z", exp.lhs()[0].name());
        assert_eq!(-1.0, exp.lhs()[0].get_data());
        assert_eq!(Relationship::LEQ, *exp.rel());
        assert_eq!("x", exp.rhs()[0].name());
        assert_eq!(-2.0, exp.rhs()[0].get_data());
        assert_eq!("y", exp.rhs()[1].name());
        assert_eq!(-3.0, exp.rhs()[1].get_data());
        assert_eq!("bonus", exp.rhs()[2].name());
        assert_eq!(-1000.0, exp.rhs()[2].get_data());
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
