/// Types of errors that may happen when using one of the algorithms.
pub enum ErrorKind {
    EulerianTrailNotFound,
    EulerianCircuitNotFound,
    NegativeCycleDetected,
}

/// Error type returns in [`algo`](crate::algo) module.
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl Error {
    /// # Arguments
    /// * `kind`: Specifies what kind of error is being created.
    /// * `msg`: Cause of the error.
    ///
    /// # Returns
    /// Constructed `Error`.
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        Error { kind, msg }
    }

    /// Creates a new [`EulerianTrailNotFound`](crate::algo::ErrorKind::EulerianTrailNotFound) kind of error.
    ///
    /// # Returns
    /// `Error` with `EulerianTrailNotFound` kind and predefined message.
    pub fn new_etnf() -> Self {
        Error {
            kind: ErrorKind::EulerianTrailNotFound,
            msg: format!("Eulerian trail not found"),
        }
    }

    /// Creates a new [`EulerianCircuitNotFound`](crate::algo::ErrorKind::EulerianCircuitNotFound) kind of error.
    ///
    /// # Returns
    /// `Error` with `EulerianCircuitNotFound` kind and predefined message.
    pub fn new_ecnf() -> Self {
        Error {
            kind: ErrorKind::EulerianCircuitNotFound,
            msg: format!("Eulerian circuit not found"),
        }
    }

    pub fn new_ncd() -> Self {
        Error {
            kind: ErrorKind::NegativeCycleDetected,
            msg: format!("Graph contains cycle"),
        }
    }

    /// # Returns
    /// Message inside of the error.
    pub fn msg(&self) -> &str {
        &self.msg
    }

    /// # Returns
    /// What kind the error is.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg())
    }
}

impl std::error::Error for Error {}
