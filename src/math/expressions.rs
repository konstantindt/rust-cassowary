use math::variables::AbstVar;
use math::variables::new_const;
use math::relationships::Relationship;

pub struct Expression {
    left_hand_side: Vec<AbstVar>,
    relationship: Relationship,
    right_hand_side: Vec<AbstVar>,
}

impl Expression {
    pub fn new(l_h_s: Vec<AbstVar>, r: Relationship, r_h_s: Vec<AbstVar>) -> Expression {
        Expression {
            left_hand_side: l_h_s,
            relationship: r,
            right_hand_side: r_h_s,
        }
    }

    pub fn lhs(&self) -> &Vec<AbstVar> {
        &self.left_hand_side
    }

    pub fn rel(&self) -> &Relationship {
        &self.relationship
    }

    pub fn rhs(&self) -> &Vec<AbstVar> {
        &self.right_hand_side
    }

    pub fn set_rel(&mut self, new_rel: Relationship) {
        self.relationship = new_rel;
    }

    pub fn add_lhs(&mut self, to_add: AbstVar) {
        self.left_hand_side.push(to_add);
    }

    pub fn move_from_lhs_side(&mut self, index: usize, insert_at_start: bool) {
        let mut to_move = self.left_hand_side.remove(index);
        to_move.change_sign();
        if self.left_hand_side.is_empty() {
            self.left_hand_side.push(new_const("zero", 0.0));
        }
        insert_side(&mut self.right_hand_side, to_move, insert_at_start);
    }

    pub fn mul_both_sides(&mut self, by: f64) {
        mul_side(&mut self.left_hand_side, by);
        mul_side(&mut self.right_hand_side, by);

        // Change sign if required
        if by.is_sign_negative() && self.relationship != Relationship::EQ {
            if self.relationship == Relationship::GEQ {
                self.relationship = Relationship::LEQ;
            } else {
                self.relationship = Relationship::GEQ;
            }
        }
    }
}

fn insert_side(side: &mut Vec<AbstVar>, var: AbstVar, start: bool) {
    // Preserve order of basic followed by non basic
    if start {
        side.insert(0, var);
    } else {
        let mut insert_at = 0;
        while insert_at < side.len() {
            match &mut side[insert_at] {
                &mut AbstVar::SlackVar { .. } => break,
                _ => insert_at += 1,
            };
        }
        side.insert(insert_at, var);
    }
}

fn mul_side(side: &mut Vec<AbstVar>, by: f64) {
    for i in 0..side.len() {
        match &mut side[i] {
            &mut AbstVar::Variable { ref mut coefficient, .. } => *coefficient = *coefficient * by,
            &mut AbstVar::Constant { ref mut value, .. } => *value = *value * by,
            _ => panic!("Unexpected variant in this program logic."),
        };
    }
}
