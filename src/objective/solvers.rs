use math::relationships::Relationship;
use objective::constraints::Constraint;
use objective::constraints::SystemOfConstraints;

pub fn transform_geq_rels(constraints: &SystemOfConstraints) {
    for constraint in constraints.system() {
        match constraint {
            &Constraint::Regular(ref ref_cell) => {
                let change: bool;
                {
                    let exp = ref_cell.borrow();
                    change = *exp.rel() == Relationship::GEQ;
                }
                if change {
                    let mut exp = ref_cell.borrow_mut();
                    exp.mul_both_sides(-1.0);
                }
            }
            &Constraint::NonNegative(_) => continue,
        };
    }
}
