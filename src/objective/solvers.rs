use math::variables::new_slack_var;
use math::relationships::Relationship;
use objective::constraints::Constraint;
use objective::constraints::SystemOfConstraints;

pub fn transform_geq_rels(constraints: &SystemOfConstraints) {
    for constraint in constraints.system() {
        match constraint {
            &Constraint::Regular(ref ref_cell) => {
                let must_change: bool;
                {
                    let exp = ref_cell.borrow();
                    must_change = *exp.rel() == Relationship::GEQ;
                }
                if must_change {
                    let mut exp = ref_cell.borrow_mut();
                    exp.mul_both_sides(-1.0);
                }
            }
            &Constraint::NonNegative(_) => continue,
        };
    }
}

pub fn transform_leq_rels(constraints: &SystemOfConstraints) {
    for i in 0..constraints.system().len() {
        match constraints.system()[i] {
            Constraint::Regular(ref ref_cell) => {
                let must_change: bool;
                {
                    let exp = ref_cell.borrow();
                    must_change = *exp.rel() == Relationship::LEQ;
                }
                if must_change {
                    let mut exp = ref_cell.borrow_mut();
                    exp.add_lhs(new_slack_var(format!("{}{}", "s", i + 1)));
                    exp.set_rel(Relationship::EQ);
                }
            }
            Constraint::NonNegative(_) => continue,
        };
    }
}
