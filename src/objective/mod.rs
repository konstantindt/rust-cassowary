pub mod functions;
pub mod problems;

#[cfg(test)]
mod tests {
    use math::variables::{AbstVar, new_var, new_const};
    use math::relationships::Relationship;
    use math::expressions::Expression;
    use objective::problems::ProblemType;
    use objective::functions::Function;

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
}
