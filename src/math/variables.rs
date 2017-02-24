use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
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

    pub fn get_data(&self) -> f64 {
        match self {
            &AbstVar::Variable { ref coefficient, .. } => *coefficient,
            &AbstVar::Constant { ref value, .. } => *value,
            &AbstVar::SlackVar { .. } => 1.0,
        }
    }

    pub fn set_data(&mut self, d: f64) {
        match self {
            &mut AbstVar::Variable { ref mut coefficient, .. } => *coefficient = d,
            &mut AbstVar::Constant { ref mut value, .. } => *value = d,
            _ => panic!("This variant does not support this method call."),
        };
    }

    pub fn change_sign(&mut self) {
        match self {
            &mut AbstVar::Variable { ref mut coefficient, .. } => *coefficient *= -1.0,
            &mut AbstVar::Constant { ref mut value, .. } => *value *= -1.0,
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
