use math::variables::new_var;
use math::relationships::Relationship;
use math::expressions::Expression;
use objective::problems::ProblemType;

pub struct Function {
    expression: Expression,
    problem_type: ProblemType,
    expression_max: Option<Expression>,
}

impl Function {
    pub fn new(e: Expression, p_t: ProblemType) -> Function {
        // Covert minimisation problems to maximasation so we can keep the Simplex in one form
        // (Maximasation Form).
        let e_m = match p_t {
            ProblemType::MAX => None,
            ProblemType::MIN => Some(create_expression_to_max(&e)),
        };
        Function {
            expression: e,
            problem_type: p_t,
            expression_max: e_m,
        }
    }

    pub fn exp(&self) -> &Expression {
        &self.expression
    }

    pub fn p_type(&self) -> &ProblemType {
        &self.problem_type
    }

    pub fn name(&self) -> String {
        if self.problem_type == ProblemType::MAX {
            let last_index = self.expression.lhs().len() - 1;
            self.expression.lhs()[last_index].name().clone()
        } else {
            self.expression.lhs()[0].name().clone()
        }
    }

    pub fn exp_max(&self) -> &Expression {
        if let Some(ref exp_to_max) = self.expression_max {
            &exp_to_max
        } else {
            &self.expression
        }
    }

    pub fn exp_max_mut(&mut self) -> &mut Expression {
        if let Some(ref mut exp_to_max) = self.expression_max {
            exp_to_max
        } else {
            &mut self.expression
        }
    }
}

fn create_expression_to_max(expression: &Expression) -> Expression {
    let original_rhs = expression.rhs();
    let mut rhs_max = Vec::with_capacity(original_rhs.len());
    for var in original_rhs {
        let mut var_clone = var.clone();
        var_clone.change_sign();
        rhs_max.push(var_clone);
    }
    Expression::new(vec![new_var("Q", 1.0)], Relationship::EQ, rhs_max)
}
