use math::variables::new_slack_var;
use math::relationships::Relationship;
use objective::constraints::Constraint;
use objective::constraints::SystemOfConstraints;

pub fn transform_leq_rels(constraints: &SystemOfConstraints) {
    for i in 0..constraints.system().len() {
        match constraints.system()[i] {
            Constraint::Regular(ref ref_cell) => {
                let mut exp = ref_cell.borrow_mut();
                if *exp.rel() == Relationship::LEQ {
                    exp.add_lhs(new_slack_var(format!("{}{}", "s", i + 1)));
                    exp.set_rel(Relationship::EQ);
                }
            }
            Constraint::NonNegative(_) => continue,
        };
    }
}
