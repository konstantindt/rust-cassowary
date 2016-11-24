#[derive(PartialEq, Debug)]
pub enum AbstVar {
    Variable { name: String, coefficient: f64 },
    Constant { name: String, value: f64 }
}

impl AbstVar {
    pub fn name(&self) -> &String {
        match self {
            &AbstVar::Variable { ref name, coefficient } => name,
            &AbstVar::Constant { ref name, value } => name,
        }
    }

    pub fn coefficient(&self) -> f64 {
        match self {
            &AbstVar::Variable { ref name, ref coefficient } => *coefficient,
            _ => panic!("This variant does not support this method call."),
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            &AbstVar::Constant { ref name, ref value } => *value,
            _ => panic!("This variant does not support this method call."),
        }
    }

    pub fn set_coefficient(&mut self, c: f64) {
        match self {
            &mut AbstVar::Variable { ref name, ref mut coefficient } => *coefficient = c,
            _ => panic!("This variant does not support this method call."),
        }
    }
}

pub fn new_var(n: &str, c: f64) -> AbstVar {
    AbstVar::Variable { name: n.to_string(), coefficient: c }
}

pub fn new_const(n: &str, v: f64) -> AbstVar {
    AbstVar::Constant { name: n.to_string(), value: v }
}
