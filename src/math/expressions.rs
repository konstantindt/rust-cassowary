use std::cell::RefCell;
use math::variables::AbstVar;
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

fn mul_side(side: &mut Vec<AbstVar>, by: f64) {
    for i in 0..side.len() {
        match &mut side[i] {
            &mut AbstVar::Variable { ref mut name, ref mut coefficient } => {
                *coefficient = *coefficient * by
            }
            &mut AbstVar::Constant { ref mut name, ref mut value } => {
                *value = *value * by
            }
        };
    }
}
