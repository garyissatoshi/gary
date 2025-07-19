#![allow(clippy::missing_safety_doc)]

#[cfg(feature = "solve")]
pub mod solver;
#[cfg(feature = "solve")]
pub use solver::solver::*;

#[cfg(feature = "solve")]
pub use crate::solver::solver::SolverMemory;
pub use equix_kernels::{Solution, SolutionByteArray, SolutionItemArray, check_all_tree_sums, Error};
#[cfg(feature = "solve")]
use hashx_cuda::{HashX, HashXBuilder};
#[cfg(not(feature = "solve"))]
use hashx::{HashX, HashXBuilder};

#[derive(Debug)]
pub struct EquiX {
    /// HashX instance generated for this puzzle's challenge string
    hash: HashX,
}

impl EquiX {
    /// Make a new [`EquiX`] instance with a challenge string and
    /// default options.
    ///
    /// It's normal for this to fail with a [`HashError::ProgramConstraints`]
    /// for a small fraction of challenge values. Those challenges must be
    /// skipped by solvers and rejected by verifiers.
    pub fn new(challenge: &[u8]) -> Result<Self, Error> {
        EquiXBuilder::new().build(challenge)
    }

    /// Check a [`Solution`] against this particular challenge.
    ///
    /// Having a [`Solution`] instance guarantees that the order of items
    /// has already been checked. This only needs to check hash tree sums.
    /// Returns either `Ok` or [`Error::HashSum`].
    pub fn verify(&self, solution: &Solution) -> Result<(), Error> {
        check_all_tree_sums(&self.hash, solution)
    }

    /// Search for solutions using this particular challenge.
    ///
    /// Returns a buffer with a variable number of solutions.
    /// Memory for the solver is allocated dynamically and not reused.
    #[cfg(feature = "solve")]
    pub fn solve(&self) -> Result<Vec<Solution>, Error> {
        let mut mem = SolverMemory::new()?;
        self.solve_with_memory(&mut mem)
    }

    /// Search for solutions, using the provided [`SolverMemory`].
    ///
    /// Returns a buffer with a variable number of solutions.
    ///
    /// Allows reuse of solver memory. Preferred for callers which may perform
    /// several solve operations in rapid succession, such as in the common case
    /// of layering an effort adjustment protocol above Equi-X.
    #[cfg(feature = "solve")]
    pub fn solve_with_memory(
        &self,
        mem: &mut SolverMemory,
    ) -> Result<Vec<Solution>, Error> {
        Ok(find_solutions(&self.hash, mem)
            .map_err(|e| Error::CudaError(e.to_string()))?)
    }
}

/// Builder for creating [`EquiX`] instances with custom settings
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EquiXBuilder {
    /// Inner [`HashXBuilder`] for options related to our hash function
    hash: HashXBuilder,
}

impl EquiXBuilder {
    /// Create a new [`EquiXBuilder`] with default settings.
    ///
    /// Immediately calling [`Self::build()`] would be equivalent to using
    /// [`EquiX::new()`].
    pub fn new() -> Self {
        Self {
            hash: HashXBuilder::new(),
        }
    }

    /// Build an [`EquiX`] instance with a challenge string and the
    /// selected options.
    ///
    /// It's normal for this to fail with a [`HashError::ProgramConstraints`]
    /// for a small fraction of challenge values. Those challenges must be
    /// skipped by solvers and rejected by verifiers.
    pub fn build(&self, challenge: &[u8]) -> Result<EquiX, Error> {
        match self.hash.build(challenge) {
            Err(e) => Err(Error::Hash(e)),
            Ok(hash) => Ok(EquiX { hash }),
        }
    }

    /// Search for solutions to a particular challenge.
    ///
    /// Each solve invocation returns zero or more solutions.
    /// Memory for the solver is allocated dynamically and not reused.
    ///
    /// It's normal for this to fail with a [`HashError::ProgramConstraints`]
    /// for a small fraction of challenge values. Those challenges must be
    /// skipped by solvers and rejected by verifiers.
    #[cfg(feature = "solve")]
    pub fn solve(&self, challenge: &[u8]) -> Result<Vec<Solution>, Error> {
        self.build(challenge)?.solve()
    }

    /// Check a [`Solution`] against a particular challenge string.
    ///
    /// Having a [`Solution`] instance guarantees that the order of items
    /// has already been checked. This only needs to check hash tree sums.
    /// Returns either `Ok` or [`Error::HashSum`].
    pub fn verify(
        &self,
        challenge: &[u8],
        solution: &Solution,
    ) -> Result<(), Error> {
        self.build(challenge)?.verify(solution)
    }

    /// Check a [`SolutionItemArray`].
    ///
    /// Returns an error if the array is not a well formed [`Solution`] or it's
    /// not suitable for the given challenge.
    pub fn verify_array(
        &self,
        challenge: &[u8],
        array: &SolutionItemArray,
    ) -> Result<(), Error> {
        // Check Solution validity before we even construct the instance
        self.verify(challenge, &Solution::try_from_array(array)?)
    }

    /// Check a [`SolutionByteArray`].
    ///
    /// Returns an error if the array is not a well formed [`Solution`] or it's
    /// not suitable for the given challenge.
    pub fn verify_bytes(
        &self,
        challenge: &[u8],
        array: &SolutionByteArray,
    ) -> Result<(), Error> {
        self.verify(challenge, &Solution::try_from_bytes(array)?)
    }
}

impl Default for EquiXBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Search for solutions, using default [`EquiXBuilder`] options.
///
/// Each solve invocation returns zero or more solutions.
/// Memory for the solver is allocated dynamically and not reused.
///
/// It's normal for this to fail with a [`HashError::ProgramConstraints`] for
/// a small fraction of challenge values. Those challenges must be skipped
/// by solvers and rejected by verifiers.
#[cfg(feature = "solve")]
pub fn solve(challenge: &[u8]) -> Result<Vec<Solution>, Error> {
    EquiX::new(challenge)?.solve()
}

/// Check a [`Solution`] against a particular challenge.
///
/// Having a [`Solution`] instance guarantees that the order of items
/// has already been checked. This only needs to check hash tree sums.
/// Returns either `Ok` or [`Error::HashSum`].
///
/// Uses default [`EquiXBuilder`] options.
pub fn verify(challenge: &[u8], solution: &Solution) -> Result<(), Error> {
    EquiX::new(challenge)?.verify(solution)
}

/// Check a [`SolutionItemArray`].
///
/// Returns an error if the array is not a well formed [`Solution`] or it's
/// not suitable for the given challenge.
///
/// Uses default [`EquiXBuilder`] options.
pub fn verify_array(
    challenge: &[u8],
    array: &SolutionItemArray,
) -> Result<(), Error> {
    // Check Solution validity before we even construct the instance
    verify(challenge, &Solution::try_from_array(array)?)
}

/// Check a [`SolutionByteArray`].
///
/// Returns an error if the array is not a well formed [`Solution`] or it's
/// not suitable for the given challenge.
///
/// Uses default [`EquiXBuilder`] options.
pub fn verify_bytes(
    challenge: &[u8],
    array: &SolutionByteArray,
) -> Result<(), Error> {
    // Check Solution validity before we even construct the instance
    verify(challenge, &Solution::try_from_bytes(array)?)
}
