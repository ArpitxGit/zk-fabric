use crate::value::{ValueId, ValueRef};

/// Errors that can occur while performing the role of an evaluator
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum EvaluatorError {
    #[error(transparent)]
    CoreError(#[from] mpz_garble_core::EvaluatorError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("context error: {0}")]
    ContextError(#[from] mpz_common::ContextError),
    // TODO: Fix the size of this error
    #[error(transparent)]
    OTError(Box<mpz_ot::OTError>),
    #[error("incorrect number of values: expected {expected}, got {actual}")]
    IncorrectValueCount { expected: usize, actual: usize },
    #[error(transparent)]
    TypeError(#[from] mpz_circuits::types::TypeError),
    #[error(transparent)]
    ValueError(#[from] mpz_garble_core::ValueError),
    #[error(transparent)]
    EncodingRegistryError(#[from] crate::memory::EncodingMemoryError),
    #[error("missing active encoding for value")]
    MissingEncoding(ValueRef),
    #[error("duplicate garbled circuit")]
    DuplicateCircuit,
    #[error("duplicate decoding for value: {0:?}")]
    DuplicateDecoding(ValueId),
    #[error(transparent)]
    VerificationError(#[from] VerificationError),
}

#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
    #[error(transparent)]
    GeneratorError(#[from] crate::generator::GeneratorError),
    #[error("invalid decoding detected")]
    InvalidDecoding,
    #[error("invalid garbled circuit detected")]
    InvalidGarbledCircuit,
}

impl From<mpz_ot::OTError> for EvaluatorError {
    fn from(err: mpz_ot::OTError) -> Self {
        Self::OTError(Box::new(err))
    }
}
