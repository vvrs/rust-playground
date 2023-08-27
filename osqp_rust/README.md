### OSQP Implementation

Objective is to provide a Rust-native implementation of OSQP algorithm that can potentially be used for the applications like MPC, Trajectory Optimization, etc.

#### How to use

Run the following command

`cargo test`

#### QP Problem

$$
\begin{align*}
\text{minimize} \quad & \frac{1}{2} x^T P x + q^T x \\
\text{subject to} \quad & l \leq Ax \leq u
\end{align*}
$$
Where:
- \( x \) is the optimization variable.
- \( P \) is a symmetric positive semidefinite matrix.
- \( q \) is a vector.
- \( A \) is a matrix.
- \( l \) and \( u \) are vectors defining the lower and upper bounds, respectively, for the linear constraints.

From the problem description,

**Inputs:** QP Params (like P, q, A matrices and l, u vectors), solver configuration (tolerances, max. iters.), and possibly initial guess (warm start)

**Output:** Optimal solution vector (x), solver status (solve, infeasible, etc.), and diagnostic info (iterations taken, final residuals, etc.)

#### **OSQP Overview**

- Convert QP to a form that is suitable for first-order methods, typically using an augmented Lagrangian for this algorithm.
- Operator Splitting
    - Decompose the problem into simpler subproblems using operator splitting
    - Main idea is to decouple the primal and dual variables -> easier to solve subproblems

- ADMM
    - Use ADMM to solve the subproblems iteratively
    - Iteration
        - Update primal variable by solving a simple qp
        - Update dual variable by using a simple linear formula
        - Update the estimates using the residuals to make sure convergence

- Convergence 
    - Convergence criteria is defined in OSQP Paper
    - Typically check that the residuals are below specified thresholds

- Linear System Solvers
    - Important steps is to solve linear systems of equations
    - Typically use Conjugate Gradient or LDLT are used based on problem structure

- Warm Start
    - Providing initial guess


#### Components 

* Core Solver - ADMM algorithm
* Linear System Solver - To solve linear systems within the ADMM iterations
* Configuration Handler - Handle solver settings and configurations
* Diagnostics & Logging
* Interface

