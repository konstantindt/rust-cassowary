pub mod variables;
pub mod relationships;

#[cfg(test)]
mod tests {
    use math::variables::{AbstVar, new_var, new_const};
    use math::relationships::Relationship;

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
}
