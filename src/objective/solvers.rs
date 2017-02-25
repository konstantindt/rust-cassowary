use math::variables::new_slack_var;
use math::relationships::Relationship;
use objective::functions::Function;
use objective::constraints::Constraint;
use objective::constraints::SystemOfConstraints;

pub fn transform_leq_rels(constraints: &SystemOfConstraints) {
    for i in 0..constraints.system().len() {
        match constraints.system()[i] {
            Constraint::Regular(ref ref_cell) => {
                let mut exp = ref_cell.borrow_mut();
                if *exp.rel() == Relationship::LEQ {
                    exp.add_lhs(new_slack_var(format!("{}{}", "sl", i + 1)));
                    exp.set_rel(Relationship::EQ);
                }
            }
            Constraint::NonNegative(_) => continue,
        };
    }
}

pub fn rearrange_fun_eq_zero(function: &mut Function) {
    let mut exp = function.exp_max().borrow_mut();
    exp.move_from_lhs_side(0, false);
    exp.swap_sides().unwrap();
    exp.mul_both_sides(-1.0);
}
