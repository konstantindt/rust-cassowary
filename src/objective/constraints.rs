use std::cell::RefCell;
use math::variables::AbstVar;
use math::expressions::Expression;

pub enum Constraint {
    Regular(RefCell<Expression>),
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
}

pub fn new_reg_con(exp: Expression) -> Constraint {
    Constraint::Regular(RefCell::new(exp))
}

pub fn new_non_neg_con(var: AbstVar) -> Constraint {
    Constraint::NonNegative(var)
}
