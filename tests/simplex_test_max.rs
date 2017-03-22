extern crate cassowary;

use cassowary::math::variables::{new_var, new_const};
use cassowary::math::relationships::Relationship;
use cassowary::math::expressions::Expression;
use cassowary::objective::functions::Function;
use cassowary::objective::problems::ProblemType;
use cassowary::objective::constraints::{new_reg_con, new_non_neg_con, SystemOfConstraints};

#[test]
fn simplex_test_max_1() {
    let exp1 = Expression::new(vec![new_var("P", 1.0)],
                               Relationship::EQ,
                               vec![new_var("x", 2.5), new_var("y", 3.0)]);
    let exp2 = Expression::new(vec![new_var("x", 3.0), new_var("y", 6.0)],
                               Relationship::LEQ,
                               vec![new_const("con1", 90.0)]);
    let exp3 = Expression::new(vec![new_var("x", 2.0), new_var("y", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con2", 35.0)]);
    let exp4 = Expression::new(vec![new_var("x", 1.0), new_var("y", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con3", 20.0)]);
    let mut objective_func = Function::new(exp1, ProblemType::MAX);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_reg_con(exp4);
    let c4 = new_non_neg_con(new_var("x", 1.0));
    let c5 = new_non_neg_con(new_var("y", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert_eq!(4, solution.len());
    assert!(solution.contains(&("P".to_string(), 55.0)));
    assert!(solution.contains(&("x".to_string(), 10.0)));
    assert!(solution.contains(&("y".to_string(), 10.0)));
    assert!(solution.contains(&("sl2".to_string(), 5.0)));
}

#[test]
fn simplex_test_max_2() {
    let exp1 = Expression::new(vec![new_var("Z", 1.0)],
                               Relationship::EQ,
                               vec![new_var("x1", 8.0), new_var("x2", 10.0), new_var("x3", 7.0)]);
    let exp2 = Expression::new(vec![new_var("x1", 1.0), new_var("x2", 3.0), new_var("x3", 2.0)],
                               Relationship::LEQ,
                               vec![new_const("con1", 10.0)]);
    let exp3 = Expression::new(vec![new_var("x1", 1.0), new_var("x2", 5.0), new_var("x3", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con2", 8.0)]);
    let mut objective_func = Function::new(exp1, ProblemType::MAX);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_non_neg_con(new_var("x1", 1.0));
    let c4 = new_non_neg_con(new_var("x2", 1.0));
    let c5 = new_non_neg_con(new_var("x3", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert_eq!(3, solution.len());
    assert!(solution.contains(&("Z".to_string(), 64.0)));
    assert!(solution.contains(&("x1".to_string(), 8.0)));
    assert!(solution.contains(&("sl1".to_string(), 2.0)));
}

#[test]
fn simplex_test_max_3() {
    let exp1 = Expression::new(vec![new_var("P", 1.0)],
                               Relationship::EQ,
                               vec![new_var("x1", 7.0), new_var("x2", 8.0), new_var("x3", 10.0)]);
    let exp2 = Expression::new(vec![new_var("x1", 2.0), new_var("x2", 3.0), new_var("x3", 2.0)],
                               Relationship::LEQ,
                               vec![new_const("con1", 1000.0)]);
    let exp3 = Expression::new(vec![new_var("x1", 1.0), new_var("x2", 1.0), new_var("x3", 2.0)],
                               Relationship::LEQ,
                               vec![new_const("con2", 800.0)]);
    let mut objective_func = Function::new(exp1, ProblemType::MAX);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_non_neg_con(new_var("x1", 1.0));
    let c4 = new_non_neg_con(new_var("x2", 1.0));
    let c5 = new_non_neg_con(new_var("x3", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert_eq!(3, solution.len());
    assert!(solution.contains(&("P".to_string(), 4400.0)));
    assert!(solution.contains(&("x1".to_string(), 200.0)));
    assert!(solution.contains(&("x3".to_string(), 300.0)));
}

#[test]
fn simplex_test_max_4() {
    let exp1 = Expression::new(vec![new_var("Z", 1.0)],
                               Relationship::EQ,
                               vec![new_var("x1", -3.0), new_var("x3", 1.0)]);
    let exp2 = Expression::new(vec![new_var("x1", 1.0),
                                    new_var("x2", 1.0),
                                    new_var("x3", 1.0),
                                    new_var("x4", 1.0)],
                               Relationship::EQ,
                               vec![new_const("con1", 4.0)]);
    let exp3 = Expression::new(vec![new_var("x1", -2.0), new_var("x2", 1.0), new_var("x3", -1.0)],
                               Relationship::EQ,
                               vec![new_const("con2", 1.0)]);
    let exp4 = Expression::new(vec![new_var("x2", 3.0), new_var("x3", 1.0), new_var("x4", 1.0)],
                               Relationship::EQ,
                               vec![new_const("con3", 9.0)]);
    let mut objective_func = Function::new(exp1, ProblemType::MAX);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_reg_con(exp4);
    let c4 = new_non_neg_con(new_var("x1", 1.0));
    let c5 = new_non_neg_con(new_var("x2", 1.0));
    let c6 = new_non_neg_con(new_var("x3", 1.0));
    let c7 = new_non_neg_con(new_var("x4", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5, c6, c7]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert_eq!(4, solution.len());
    assert!(solution.contains(&("Z".to_string(), 1.5)));
    assert!(solution.contains(&("x2".to_string(), 2.5)));
    assert!(solution.contains(&("x3".to_string(), 1.5)));
    assert!(solution.contains(&("x4".to_string(), 0.0)));
}

#[test]
fn simplex_test_max_5() {
    let exp1 = Expression::new(vec![new_var("P", 1.0)],
                               Relationship::EQ,
                               vec![new_var("x1", 1.0), new_var("x2", -1.0), new_var("x3", 3.0)]);
    let exp2 = Expression::new(vec![new_var("x1", 1.0), new_var("x2", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con1", 20.0)]);
    let exp3 = Expression::new(vec![new_var("x1", 1.0), new_var("x3", 1.0)],
                               Relationship::EQ,
                               vec![new_const("con2", 5.0)]);
    let exp4 = Expression::new(vec![new_var("x2", 1.0), new_var("x3", 1.0)],
                               Relationship::GEQ,
                               vec![new_const("con3", 10.0)]);
    let mut objective_func = Function::new(exp1, ProblemType::MAX);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_reg_con(exp4);
    let c4 = new_non_neg_con(new_var("x1", 1.0));
    let c5 = new_non_neg_con(new_var("x2", 1.0));
    let c6 = new_non_neg_con(new_var("x3", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5, c6]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert!(solution.contains(&("P".to_string(), 10.0)));
    assert!(solution.contains(&("x2".to_string(), 5.0)));
    assert!(solution.contains(&("x3".to_string(), 5.0)));
    assert!(solution.contains(&("sl1".to_string(), 15.0)));
}

#[test]
fn simplex_test_max_6() {
    let exp1 = Expression::new(vec![new_var("F", 1.0)],
                               Relationship::EQ,
                               vec![new_var("x1", 2.0), new_var("x2", 3.0), new_var("x3", 4.0)]);
    let exp2 = Expression::new(vec![new_var("x1", 3.0), new_var("x2", 1.0), new_var("x3", 4.0)],
                               Relationship::LEQ,
                               vec![new_const("con1", 600.0)]);
    let exp3 = Expression::new(vec![new_var("x1", 2.0), new_var("x2", 4.0), new_var("x3", 2.0)],
                               Relationship::GEQ,
                               vec![new_const("con2", 480.0)]);
    let exp4 = Expression::new(vec![new_var("x1", 2.0), new_var("x2", 3.0), new_var("x3", 3.0)],
                               Relationship::EQ,
                               vec![new_const("con3", 540.0)]);
    let mut objective_func = Function::new(exp1, ProblemType::MAX);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_reg_con(exp4);
    let c4 = new_non_neg_con(new_var("x1", 1.0));
    let c5 = new_non_neg_con(new_var("x2", 1.0));
    let c6 = new_non_neg_con(new_var("x3", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5, c6]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert!(solution.contains(&("F".to_string(), 660.0)));
    assert!(solution.contains(&("x2".to_string(), 60.000004)));
    assert!(solution.contains(&("x3".to_string(), 120.0)));
    assert!(solution.contains(&("sl1".to_string(), 59.99998)));
}
