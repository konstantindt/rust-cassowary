extern crate cassowary;

use cassowary::math::variables::{new_var, new_const};
use cassowary::math::relationships::Relationship;
use cassowary::math::expressions::Expression;
use cassowary::objective::functions::Function;
use cassowary::objective::problems::ProblemType;
use cassowary::objective::constraints::{new_reg_con, new_non_neg_con, SystemOfConstraints};

#[test]
fn simplex_test_min_1() {
    // Create a snack composed of: bread, milk, cheese, potato, fish, yogurt.
    // Minimise the cost but certain dietary requirements have to be satisfied.
    //
    // Minimise 2b + 3.5m + 8c + 1.5p + 11f + y (the cost of the snack)
    let exp1 = Expression::new(vec![new_var("C", 1.0)],
                               Relationship::EQ,
                               vec![new_var("b", 2.0),
                                    new_var("m", 3.5),
                                    new_var("c", 8.0),
                                    new_var("p", 1.5),
                                    new_var("f", 11.0),
                                    new_var("y", 1.0)]);
    // Subject to:
    // 4b + 8m + 7c + 1.3p + 8f + 9.2y ≤ 10 (proteins)
    let exp2 = Expression::new(vec![new_var("b", 4.0),
                                    new_var("m", 8.0),
                                    new_var("c", 7.0),
                                    new_var("p", 1.3),
                                    new_var("f", 8.0),
                                    new_var("y", 9.2)],
                               Relationship::LEQ,
                               vec![new_const("con1", 10.0)]);
    // b + 5m + 9c + 0.1p + 7f + 1y ≥ 8 (fat)
    let exp3 = Expression::new(vec![new_var("b", 1.0),
                                    new_var("m", 5.0),
                                    new_var("c", 9.0),
                                    new_var("p", 0.1),
                                    new_var("f", 7.0),
                                    new_var("y", 1.0)],
                               Relationship::GEQ,
                               vec![new_const("con2", 8.0)]);
    // 15b + 11.7m + 0.4c + 22.6p + 17y ≥ 10 (carbohydrates)
    let exp4 = Expression::new(vec![new_var("b", 15.0),
                                    new_var("m", 11.7),
                                    new_var("c", 0.4),
                                    new_var("p", 22.6),
                                    new_var("y", 17.0)],
                               Relationship::GEQ,
                               vec![new_const("con3", 10.0)]);
    // 90b + 120m + 106c + 97p + 130f + 180y ≥ 150 (calories)
    let exp5 = Expression::new(vec![new_var("b", 90.0),
                                    new_var("m", 120.0),
                                    new_var("c", 106.0),
                                    new_var("p", 97.0),
                                    new_var("f", 130.0),
                                    new_var("y", 180.0)],
                               Relationship::GEQ,
                               vec![new_const("con4", 150.0)]);
    let exp6 = Expression::new(vec![new_var("m", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con5", 1.0)]);
    let exp7 = Expression::new(vec![new_var("f", 1.0)],
                               Relationship::GEQ,
                               vec![new_const("con5", 0.5)]);
    let mut objective_func = Function::new(exp1, ProblemType::MIN);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_reg_con(exp4);
    let c4 = new_reg_con(exp5);
    let c5 = new_reg_con(exp6);
    let c6 = new_reg_con(exp7);
    let c7 = new_non_neg_con(new_var("b", 1.0));
    let c8 = new_non_neg_con(new_var("m", 1.0));
    let c9 = new_non_neg_con(new_var("c", 1.0));
    let c10 = new_non_neg_con(new_var("p", 1.0));
    let c11 = new_non_neg_con(new_var("f", 1.0));
    let c12 = new_non_neg_con(new_var("y", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11,
                                                   c12]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert_eq!(13, solution.len());
    assert!(solution.contains(&("C".to_string(), 9.174414)));
    assert!(solution.contains(&("b".to_string(), 0.0)));
    assert!(solution.contains(&("m".to_string(), 0.5644002)));
    assert!(solution.contains(&("c".to_string(), 0.18481092)));
    assert!(solution.contains(&("p".to_string(), 0.14701743)));
    assert!(solution.contains(&("f".to_string(), 0.49999994)));
    assert!(solution.contains(&("y".to_string(), 0.0)));
    assert!(solution.contains(&("sl1".to_string(), 0.0)));
    assert!(solution.contains(&("su2".to_string(), 0.0)));
    assert!(solution.contains(&("su3".to_string(), 0.0)));
    assert!(solution.contains(&("su4".to_string(), 16.57866)));
    assert!(solution.contains(&("sl5".to_string(), 0.43559977)));
    assert!(solution.contains(&("su6".to_string(), 0.0)));
}

#[test]
fn simplex_test_min_2() {
    let exp1 = Expression::new(vec![new_var("P", 1.0)],
                               Relationship::EQ,
                               vec![new_var("x", 4.0), new_var("y", -5.0), new_var("z", -3.0)]);
    let exp2 = Expression::new(vec![new_var("x", 1.0), new_var("y", -1.0), new_var("z", 1.0)],
                               Relationship::GEQ,
                               vec![new_const("con1", -2.0)]);
    let exp3 = Expression::new(vec![new_var("x", 1.0), new_var("y", 1.0), new_var("z", 2.0)],
                               Relationship::LEQ,
                               vec![new_const("con2", 3.0)]);
    let mut objective_func = Function::new(exp1, ProblemType::MIN);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_non_neg_con(new_var("x", 1.0));
    let c4 = new_non_neg_con(new_var("y", 1.0));
    let c5 = new_non_neg_con(new_var("z", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert_eq!(6, solution.len());
    assert!(solution.contains(&("P".to_string(), -38.0 / 3.0)));
    assert!(solution.contains(&("x".to_string(), 0.0)));
    assert!(solution.contains(&("y".to_string(), 7.0 / 3.0)));
    assert!(solution.contains(&("z".to_string(), 1.0 / 3.0)));
    assert!(solution.contains(&("sl1".to_string(), 0.0)));
    assert!(solution.contains(&("sl2".to_string(), 0.0)));
}

#[test]
fn simplex_test_min_3() {
    let exp1 = Expression::new(vec![new_var("W", 1.0)],
                               Relationship::EQ,
                               vec![new_var("y1", 3.0), new_var("y2", 2.0), new_var("y3", 3.0)]);
    let exp2 = Expression::new(vec![new_var("y1", 2.0), new_var("y2", 3.0), new_var("y3", 6.0)],
                               Relationship::LEQ,
                               vec![new_const("con1", 60.0)]);
    let exp3 = Expression::new(vec![new_var("y1", 1.0), new_var("y2", 4.0), new_var("y3", 5.0)],
                               Relationship::GEQ,
                               vec![new_const("con2", 40.0)]);
    let mut objective_func = Function::new(exp1, ProblemType::MIN);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_non_neg_con(new_var("y1", 1.0));
    let c4 = new_non_neg_con(new_var("y2", 1.0));
    let c5 = new_non_neg_con(new_var("y3", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert_eq!(6, solution.len());
    assert!(solution.contains(&("W".to_string(), 20.0)));
    assert!(solution.contains(&("y1".to_string(), 0.0)));
    assert!(solution.contains(&("y2".to_string(), 10.0)));
    assert!(solution.contains(&("y3".to_string(), 0.0)));
    assert!(solution.contains(&("sl1".to_string(), 30.0)));
    assert!(solution.contains(&("su2".to_string(), 0.0)));
}

#[test]
fn simplex_test_min_4() {
    let exp1 = Expression::new(vec![new_var("C", 1.0)],
                               Relationship::EQ,
                               vec![new_var("x1", 3.0), new_var("x2", 9.0)]);
    let exp2 = Expression::new(vec![new_var("x1", 2.0), new_var("x2", 1.0)],
                               Relationship::GEQ,
                               vec![new_const("con1", 8.0)]);
    let exp3 = Expression::new(vec![new_var("x1", 1.0), new_var("x2", 2.0)],
                               Relationship::GEQ,
                               vec![new_const("con2", 8.0)]);
    let mut objective_func = Function::new(exp1, ProblemType::MIN);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_non_neg_con(new_var("x1", 1.0));
    let c4 = new_non_neg_con(new_var("x2", 1.0));
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    assert_eq!(5, solution.len());
    assert!(solution.contains(&("C".to_string(), 24.0)));
    assert!(solution.contains(&("x1".to_string(), 8.0)));
    assert!(solution.contains(&("x2".to_string(), 0.0)));
    assert!(solution.contains(&("su1".to_string(), 8.0)));
    assert!(solution.contains(&("su2".to_string(), 0.0)));
}
