use math::variables::{new_slack_var, new_surplus_var};
use math::relationships::Relationship;
use objective::functions::Function;
use objective::constraints::{Constraint, SystemOfConstraints};

pub fn transform_constraint_rels_to_eq(constraints: &SystemOfConstraints) {
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
                    _ => continue,
                }
            }
            &Constraint::NonNegative(_) => continue,
        };
    }
}

pub fn rearrange_fun_eq_zero(function: &mut Function) {
    let mut exp = function.exp_max().borrow_mut();
    exp.move_from_lhs_side(0, false);
    exp.swap_sides().unwrap();
    exp.mul_both_sides(-1.0);
}
