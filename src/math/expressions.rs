use std::mem;
use std::result::Result;
use std::fmt;
use Num;
use math::variables::{AbstVar, new_const};
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
        add_side(&mut self.left_hand_side, to_add);
    }

    pub fn add_rhs(&mut self, to_add: AbstVar) {
        add_side(&mut self.right_hand_side, to_add);
    }

    pub fn move_from_lhs_side(&mut self, index: usize, insert_at_start: bool) {
        let mut to_move = self.left_hand_side.remove(index);
        to_move.change_sign();
        if self.left_hand_side.is_empty() {
            self.left_hand_side.push(new_const("RHS", 0.0));
        }
        insert_side(&mut self.right_hand_side, to_move, insert_at_start);
    }

    pub fn mul_both_sides(&mut self, by: Num) {
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

    pub fn swap_sides(&mut self) -> Result<&str, &str> {
        if self.relationship == Relationship::EQ {
            mem::swap(&mut self.left_hand_side, &mut self.right_hand_side);
            Ok("Swapped!")
        } else {
            Err("Invalid state: relationship must be \"EQ\".")
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Expression")
            .field("lhs", &self.left_hand_side)
            .field("rel", &self.relationship)
            .field("rhs", &self.right_hand_side)
            .finish()
    }
}

fn add_side(side: &mut Vec<AbstVar>, to_add: AbstVar) {
    if let Some(new_var) = collect_into_existing(side, to_add) {
        side.push(new_var);
    }
}

fn collect_into_existing(side: &mut Vec<AbstVar>, to_add: AbstVar) -> Option<AbstVar> {
    for mut var in side.iter_mut() {
        if var == &to_add {
            let old_var_data = var.get_data();
            var.set_data(old_var_data + to_add.get_data());
            return None;
        }
    }
    Some(to_add)
}

fn insert_side(side: &mut Vec<AbstVar>, var: AbstVar, start: bool) {
    // Maybe variable already exits...
    if let Some(to_insert) = collect_into_existing(side, var) {
        // ... or not.
        // Preserve order of decision variables followed by non-decision variables.
        if start {
            side.insert(0, to_insert);
        } else {
            let mut insert_at = 0;
            while insert_at < side.len() {
                match &side[insert_at] {
                    &AbstVar::Variable { .. } => insert_at += 1,
                    _ => break,
                };
            }
            side.insert(insert_at, to_insert);
        }
    }
}

fn mul_side(side: &mut Vec<AbstVar>, by: Num) {
    for var in side.iter_mut() {
        match var {
            &mut AbstVar::Variable { ref mut coefficient, .. } => *coefficient = *coefficient * by,
            &mut AbstVar::Constant { ref mut value, .. } => *value = *value * by,
            _ => panic!("Unexpected variant in this program logic."),
        };
    }
}
