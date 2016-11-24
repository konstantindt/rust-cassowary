#[derive(PartialEq, Debug)]
pub enum AbstVar {
    Variable { name: String, coefficient: f64 }
}

impl AbstVar {
    pub fn name(&self) -> &String {
        match self {
            &AbstVar::Variable { ref name, coefficient } => name,
        }
    }

    pub fn coefficient(&self) -> f64 {
        match self {
            &AbstVar::Variable { ref name, ref coefficient } => *coefficient,
        }
    }
}

pub fn new_var(n: &str, c: f64) -> AbstVar {
    AbstVar::Variable { name: n.to_string(), coefficient: c }
}
