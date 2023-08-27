use osqp_rust::interface::inputs::{ProblemData, SolverSettings};
use osqp_rust::utils::CscMatrix;
use osqp_rust::core_solver::Problem;

fn main() {
    // Define problem data using the dedicated struct from the interface module
    let problem_data = ProblemData {
        P: &[[4.0, 1.0],
             [1.0, 2.0]],
        q: &[1.0, 1.0],
        A: &[[1.0, 1.0],
             [1.0, 0.0],
             [0.0, 1.0]],
        l: &[1.0, 0.0, 0.0],
        u: &[1.0, 0.7, 0.7],
    };

    // Extract the upper triangular elements of `P` using utility functions
    let P_upper_tri = CscMatrix::from_upper_tri(&problem_data.P);

    // Define solver settings using the dedicated struct from the interface module
    let settings = SolverSettings::default().verbose(false);

    // Create an OSQP problem using the core solver module
    let mut prob = Problem::new(&P_upper_tri, &problem_data, &settings).expect("failed to setup problem");

    // Solve the problem
    let result = prob.solve();

    // Print the solution
    println!("{:?}", result.x().expect("failed to solve problem"));
}
