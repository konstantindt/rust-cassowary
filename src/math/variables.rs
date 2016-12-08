use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum AbstVar {
    Variable { name: String, coefficient: f64 },
    Constant { name: String, value: f64 },
    SlackVar { name: String },
}

impl Hash for AbstVar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for AbstVar {
    fn eq(&self, other: &AbstVar) -> bool {
        match (self, other) {
            (&AbstVar::Variable { name: ref n1, .. }, &AbstVar::Variable { name: ref n2, .. }) =>
                *n1 == *n2,
            (&AbstVar::Constant { name: ref n1, .. }, &AbstVar::Constant { name: ref n2, .. }) =>
                *n1 == *n2,
            (&AbstVar::SlackVar { name: ref n1 }, &AbstVar::SlackVar { name: ref n2 }) =>
                *n1 == *n2,
            _ => false,
        }
    }
}

impl Eq for AbstVar {}

impl AbstVar {
    pub fn name(&self) -> &String {
        match self {
            &AbstVar::Variable { ref name, .. } => name,
            &AbstVar::Constant { ref name, .. } => name,
            &AbstVar::SlackVar { ref name } => name,
        }
    }

    pub fn coefficient(&self) -> f64 {
        match self {
            &AbstVar::Variable { ref coefficient, .. } => *coefficient,
            _ => panic!("This variant does not support this method call."),
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            &AbstVar::Constant { ref value, .. } => *value,
            _ => panic!("This variant does not support this method call."),
        }
    }

    pub fn set_coefficient(&mut self, c: f64) {
        match self {
            &mut AbstVar::Variable { ref mut coefficient, .. } => *coefficient = c,
            _ => panic!("This variant does not support this method call."),
        }
    }

    pub fn set_value(&mut self, v: f64) {
        match self {
            &mut AbstVar::Constant { ref mut value, .. } => *value = v,
            _ => panic!("This variant does not support this method call."),
        }
    }
}

pub fn new_var(n: &str, c: f64) -> AbstVar {
    AbstVar::Variable {
        name: n.to_string(),
        coefficient: c,
    }
}

pub fn new_const(n: &str, v: f64) -> AbstVar {
    AbstVar::Constant {
        name: n.to_string(),
        value: v,
    }
}

pub fn new_slack_var(n: String) -> AbstVar {
    AbstVar::SlackVar { name: n }
}
