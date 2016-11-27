use math::expressions::Expression;
use objective::problems::ProblemType;

pub struct Function {
    expression: Expression,
    problem_type: ProblemType,
}

impl Function {
    pub fn new(e: Expression, p_t: ProblemType) -> Function {
        Function {
            expression: e,
            problem_type: p_t,
        }
    }

    pub fn exp(&self) -> &Expression {
        &self.expression
    }

    pub fn p_type(&self) -> &ProblemType {
        &self.problem_type
    }
}
