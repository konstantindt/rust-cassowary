use math::variables::{AbstVar, new_var, new_const, new_slack_var, new_surplus_var, new_arti_var};
use math::expressions::Expression;
use math::relationships::Relationship;
use objective::functions::Function;
use objective::problems::ProblemType;
use objective::constraints::{Constraint, SystemOfConstraints};

pub fn transform_constraint_rels_to_eq(constraints: &SystemOfConstraints) -> Option<Function> {
    let mut phase1: Option<Expression> = None;
    for (i, constraint) in constraints.system().iter().enumerate() {
        match constraint {
            &Constraint::Regular(ref ref_cell) => {
                let mut exp = ref_cell.borrow_mut();
                if exp.rhs()[0].get_data().is_sign_negative() {
                    // Negative constants on the right hand side are not allowed.
                    exp.mul_both_sides(-1.0);
                }
                match exp.rel() {
                    &Relationship::LEQ => {
                        exp.add_lhs(new_slack_var(format!("{}{}", "sl", i + 1)));
                        exp.set_rel(Relationship::EQ);
                    }
                    &Relationship::GEQ => {
                        exp.add_lhs(new_surplus_var(format!("{}{}", "su", i + 1)));
                        exp.set_rel(Relationship::EQ);
                    }
                    &Relationship::EQ => {
                        // Build function for phase 1.
                        if let Some(ref mut phase1_fun_exp) = phase1 {
                            phase1_fun_exp.add_rhs(new_const("RHS",
                                                             -1.0 * exp.rhs()[0].get_data()));
                            for var in exp.lhs() {
                                phase1_fun_exp.add_rhs(var.clone());
                            }
                        } else {
                            let mut phase1_fun_exp = Expression::new(vec![new_var("W", 1.0)],
                                                                     Relationship::EQ,
                                                                     exp.lhs().clone());
                            phase1_fun_exp.add_rhs(new_const("RHS",
                                                             -1.0 * exp.rhs()[0].get_data()));
                            phase1 = Some(phase1_fun_exp);
                        }
                        // Transform.
                        exp.add_lhs(new_arti_var(format!("{}{}", "arti", i + 1)));
                    }
                }
            }
            &Constraint::NonNegative(_) => continue,
        };
    }
    if let Some(phase1_fun_exp) = phase1 {
        Some(Function::new(phase1_fun_exp, ProblemType::MAX))
    } else {
        None
    }
}

pub fn rearrange_fun_eq_zero(function: &mut Function) {
    let mut exp = function.exp_max().borrow_mut();
    exp.move_from_lhs_side(0, false);
    exp.swap_sides().unwrap();
    // Move the constant on the other side if present.
    let search = exp.lhs().iter().rposition(|var| match var {
                                                &AbstVar::Constant { .. } => true,
                                                _ => false,
                                            });
    if let Some(found_index) = search {
        exp.move_from_lhs_side(found_index, false);
    }
    exp.mul_both_sides(-1.0);
}
