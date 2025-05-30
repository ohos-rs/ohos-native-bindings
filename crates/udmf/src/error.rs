#[derive(Debug)]
pub enum UdmfError {
    /// Call system api failed
    InternalError(i32),

    /// Create a new Utd failed
    UtdInitError(String),

    /// Init Uds failed
    UdsInitError(String),

    /// Init udmf failed
    UdmfInitError(String),

    /// Some STD called error
    CommonError(String),
}

impl std::fmt::Display for UdmfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UdmfError::InternalError(err) => write!(f, "Call system api failed: {}", err),
            UdmfError::UtdInitError(err) => {
                write!(f, "Create a new Utd with type {:?} failed.", err)
            }
            UdmfError::UdsInitError(err) => write!(f, "Init {} Uds failed.", err),
            UdmfError::UdmfInitError(err) => write!(f, "Init udmf failed: {}", err),
            UdmfError::CommonError(err) => write!(f, "Std Error: {}", err),
        }
    }
}

impl std::error::Error for UdmfError {}
