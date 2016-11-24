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
}
