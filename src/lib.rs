pub mod math;
pub mod objective;
pub mod tableau;

use math::variables::is_gen_arti_var;
use objective::problems::ProblemType;
use objective::functions::Function;
use objective::constraints::SystemOfConstraints;
use objective::solvers::{transform_constraint_rels_to_eq, rearrange_fun_eq_zero};
use tableau::tables::Table;
use tableau::initials::{get_initial_table_from, append_function};
use tableau::enter_vars::{enter_var_pivot_optimal, enter_var_pivot_feasible};
use tableau::leave_vars::leave_var;
use tableau::pivots::{pivot_around, apply_transition_rule};

pub type Num = f32;

pub fn optimise(function: &mut Function, constraints: &SystemOfConstraints) -> Vec<(String, Num)> {
    rearrange_fun_eq_zero(function);
    if let Some(mut phase1_fun) = transform_constraint_rels_to_eq(constraints) {
        rearrange_fun_eq_zero(&mut phase1_fun);
        let mut phase1_table = get_initial_table_from(function, constraints);
        // Set Phase I function to work with.
        append_function(&phase1_fun, &mut phase1_table);
        let phase1_solution = run_simplex(&phase1_fun, &mut phase1_table);
        if phase1_solution.contains(&("W".to_string(), 0.0)) {
            // Check to see if there are any artificial variables in the Phase I solution.
            let arti_vars_in_solution = phase1_solution.into_iter()
                .filter(|basic_var| is_gen_arti_var(&basic_var.0))
                .collect::<Vec<(String, Num)>>();
            if arti_vars_in_solution.is_empty() {
                // Carry out Phase II - no need for Transition Rule.
                return run_phase_2_from_1(function, &mut phase1_table);
            } else {
                // Remove artificial variables from the basis by applying the Transition Rule.
                apply_transition_rule(arti_vars_in_solution, constraints, &mut phase1_table);
                return run_phase_2_from_1(function, &mut phase1_table);
            }
        } else {
            panic!("Could not find a feasible solution to start Phase II.");
        }
    } else {
        // Carry on with Phase II.
        let mut table = get_initial_table_from(function, constraints);
        return run_simplex(function, &mut table);
    }
}

fn run_simplex(function: &Function, table: &mut Table) -> Vec<(String, Num)> {
    loop {
        match table.get_basic_solution() {
            Ok(mut basic_solution) => {
                if table.is_solution_optimal() {
                    if function.p_type() == &ProblemType::MIN {
                        // Give solution for MIN as currently it is given as MAX.
                        let (pos, _) =
                            basic_solution.iter()
                                .enumerate()
                                .find(|&entry| (entry.1).0 == "Q")
                                .expect("Failed to locate value of \"Q\" in optimal solution.");
                        basic_solution[pos] = (function.name().clone(),
                                               basic_solution[pos].1 * -1.0);
                        return basic_solution;
                    } else {
                        return basic_solution;
                    }
                } else {
                    let enter_var_index = enter_var_pivot_optimal(&table);
                    pivot_around(enter_var_index, leave_var(enter_var_index, &table), table);
                }
            }
            Err(index_report) => {
                let enter_var_index = enter_var_pivot_feasible(&table,
                                                               index_report.0,
                                                               index_report.1)
                                                               .expect("Could not find a leftmost \
                                                               positive value cell for pivoting \
                                                               to enter feasible region.");
                pivot_around(enter_var_index, leave_var(enter_var_index, &table), table);
            }
        }
    }
}

fn run_phase_2_from_1(fun: &Function, table: &mut Table) -> Vec<(String, Num)> {
    // Set original function to work with.
    table.remove_last_row();
    let old_num_fun_rows = table.get_num_fun_rows();
    table.set_num_fun_rows(old_num_fun_rows - 1);
    run_simplex(fun, table)
}
