use crate::util::{Curve, Domain, Expression, Query};

#[cfg(feature = "halo2")]
pub mod halo2;

#[derive(Clone, Debug)]
pub struct Protocol<C: Curve> {
    pub zk: bool,
    pub domain: Domain<C::Scalar>,
    pub preprocessed: Vec<C>,
    pub num_statement: usize,
    pub num_auxiliary: Vec<usize>,
    pub num_challenge: Vec<usize>,
    pub evaluations: Vec<Query>,
    pub queries: Vec<Query>,
    pub relations: Vec<Expression<C::Scalar>>,
    pub transcript_initial_state: C::Scalar,
    pub accumulator_indices: Option<Vec<Vec<(usize, usize)>>>,
}

impl<C: Curve> Protocol<C> {
    pub fn vanishing_poly(&self) -> usize {
        self.preprocessed.len() + self.num_statement + self.num_auxiliary.iter().sum::<usize>()
    }
}
