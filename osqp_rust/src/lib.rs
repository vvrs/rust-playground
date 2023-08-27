mod osqp {
    pub mod proximal_operator;
    pub mod admm_update;
}

#[cfg(test)]

mod tests {
    use super::osqp::proximal_operator::proximal_operator;
    use super::osqp::admm_update::admm_update;

    #[test]
    fn test_proximal_operator() {
        // TODO: Implement
        // assert_eq!(proximal_operator(), core::option::Option<()>);
    }

    #[test]
    fn test_admm_update() {
        // TODO: Implement and test
        // assert_eq!(admm_update(), core::option::Option<()>);
    }
}