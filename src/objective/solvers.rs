use std::ops::DerefMut;
use math::variables::{new_slack_var, new_surplus_var};
use math::relationships::Relationship;
use math::expressions::Expression;
use objective::functions::Function;
use objective::constraints::{Constraint, SystemOfConstraints};

pub fn transform_constraint_rels_to_eq(constraints: &SystemOfConstraints) {
    for (i, constraint) in constraints.system().iter().enumerate() {
        match constraint {
            &Constraint::Regular(ref ref_cell) => {
                let mut exp = ref_cell.borrow_mut();
                if *exp.rel() == Relationship::LEQ {
                    add_slack_var(exp.deref_mut(), i);
                } else if *exp.rel() == Relationship::GEQ {
                    if !exp.rhs()[0].get_data().is_sign_negative() {
                        exp.add_lhs(new_surplus_var(format!("{}{}", "su", i + 1)));
                        exp.set_rel(Relationship::EQ);
                    } else {
                        // Transform it first to a less than or equal to constraint.
                        exp.mul_both_sides(-1.0);
                        add_slack_var(exp.deref_mut(), i);
                    }
                }
            }
            &Constraint::NonNegative(_) => continue,
        };
    }
}

fn add_slack_var(exp: &mut Expression, label_num: usize) {
    exp.add_lhs(new_slack_var(format!("{}{}", "sl", label_num + 1)));
    exp.set_rel(Relationship::EQ);
}

pub fn rearrange_fun_eq_zero(function: &mut Function) {
    let mut exp = function.exp_max().borrow_mut();
    exp.move_from_lhs_side(0, false);
    exp.swap_sides().unwrap();
    exp.mul_both_sides(-1.0);
}
