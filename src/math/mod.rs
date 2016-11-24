pub mod variables;

#[cfg(test)]
mod tests {
    use math::variables::{AbstVar, new_var};

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
}
