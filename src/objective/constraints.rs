use math::variables::AbstVar;
use math::expressions::Expression;

#[derive(Clone)]
pub enum Constraint {
    Regular(Expression),
    NonNegative(AbstVar),
}

pub struct SystemOfConstraints {
    constraints: Vec<Constraint>,
}

impl SystemOfConstraints {
    pub fn new(c: Vec<Constraint>) -> SystemOfConstraints {
        SystemOfConstraints { constraints: c }
    }

    pub fn system(&self) -> &Vec<Constraint> {
        &self.constraints
    }

    pub fn system_mut(&mut self) -> &mut Vec<Constraint> {
        &mut self.constraints
    }
}

pub fn new_reg_con(exp: Expression) -> Constraint {
    Constraint::Regular(exp)
}

pub fn new_non_neg_con(var: AbstVar) -> Constraint {
    Constraint::NonNegative(var)
}
