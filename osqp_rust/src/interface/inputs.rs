use nalgebra::{DMatrix, DVector};

pub struct ProblemData {
    pub n: usize,   // Number of variables
    pub m: usize,   // Number of constraints
    pub P: DMatrix<f64>,   // Cost function matrix
    pub q: DVector<f64>,   // Cost function vector
    pub A: DMatrix<f64>,   // Linear constraints matrix
    pub l: DVector<f64>,   // Lower bounds vector
    pub u: DVector<f64>,   // Upper bounds vector
}

pub struct SolverSettings {
    pub verbose: bool,
}

impl Default for SolverSettings {
    fn default() -> Self {
        SolverSettings {
            verbose: true,
        }
    }
}