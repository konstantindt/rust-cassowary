use std::cell::RefCell;
use math::variables::new_var;
use math::relationships::Relationship;
use math::expressions::Expression;
use objective::problems::ProblemType;

pub struct Function {
    expression: RefCell<Expression>,
    problem_type: ProblemType,
    expression_max: Option<RefCell<Expression>>,
}

impl Function {
    pub fn new(e: Expression, p_t: ProblemType) -> Function {
        let mut e_m = None;
        if p_t == ProblemType::MIN {
            e_m = Some(RefCell::new(create_expression_to_max(&e)));
        }
        Function {
            expression: RefCell::new(e),
            problem_type: p_t,
            expression_max: e_m,
        }
    }

    pub fn exp(&self) -> &RefCell<Expression> {
        &self.expression
    }

    pub fn p_type(&self) -> &ProblemType {
        &self.problem_type
    }

    pub fn exp_max(&self) -> &RefCell<Expression> {
        if let Some(ref exp_to_max) = self.expression_max {
            &exp_to_max
        } else {
            &self.expression
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
