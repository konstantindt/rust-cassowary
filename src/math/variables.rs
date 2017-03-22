use std::hash::{Hash, Hasher};
use Num;

#[derive(Debug, Clone)]
pub enum AbstVar {
    Variable { name: String, coefficient: Num },
    Constant { name: String, value: Num },
    SlackVar { name: String },
    SurplusVar { name: String },
    ArtiVar { name: String },
}

impl Hash for AbstVar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for AbstVar {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn eq(&self, other: &AbstVar) -> bool {
        match (self, other) {
            (&AbstVar::Variable { name: ref n1, .. }, &AbstVar::Variable { name: ref n2, .. }) =>
                *n1 == *n2,
            (&AbstVar::Constant { name: ref n1, .. }, &AbstVar::Constant { name: ref n2, .. }) =>
                *n1 == *n2,
            (&AbstVar::SlackVar { name: ref n1 }, &AbstVar::SlackVar { name: ref n2 }) =>
                *n1 == *n2,
            (&AbstVar::SurplusVar { name: ref n1 }, &AbstVar::SurplusVar { name: ref n2 }) =>
                *n1 == *n2,
            (&AbstVar::ArtiVar { name: ref n1 }, &AbstVar::ArtiVar { name: ref n2 }) =>
                *n1 == *n2,
            _ => false,
        }
    }
}

impl Eq for AbstVar {}

impl AbstVar {
    pub fn name(&self) -> &String {
        match self {
            &AbstVar::Variable { ref name, .. } |
            &AbstVar::Constant { ref name, .. } => name,
            &AbstVar::SlackVar { ref name } |
            &AbstVar::SurplusVar { ref name } |
            &AbstVar::ArtiVar { ref name } => name,
        }
    }

    pub fn get_data(&self) -> Num {
        match self {
            &AbstVar::Variable { ref coefficient, .. } => *coefficient,
            &AbstVar::Constant { ref value, .. } => *value,
            &AbstVar::SlackVar { .. } |
            &AbstVar::ArtiVar { .. } => 1.0,
            &AbstVar::SurplusVar { .. } => -1.0,
        }
    }

    pub fn set_data(&mut self, d: Num) {
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

pub fn new_var(n: &str, c: Num) -> AbstVar {
    AbstVar::Variable {
        name: n.to_string(),
        coefficient: c,
    }
}

pub fn new_const(n: &str, v: Num) -> AbstVar {
    AbstVar::Constant {
        name: n.to_string(),
        value: v,
    }
}

pub fn new_slack_var(n: String) -> AbstVar {
    AbstVar::SlackVar { name: n }
}

pub fn new_surplus_var(n: String) -> AbstVar {
    AbstVar::SurplusVar { name: n }
}

pub fn new_arti_var(n: String) -> AbstVar {
    AbstVar::ArtiVar { name: n }
}

pub fn is_gen_arti_var(name: &String) -> bool {
    if name.len() < 4 {
        return false;
    }
    let (part1, part2) = name.split_at(4);
    if part1 != "arti" {
        return false;
    } else {
        match part2.parse::<usize>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
