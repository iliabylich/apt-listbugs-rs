#[derive(Debug, PartialEq)]
pub(crate) enum AppError {
    NoAptHookInfoFdVariable,
    InvalidAptHookInfoVariable,
    CantReadFromAptHook,
    NoNewlineBetweenAptHookParts,
    InvalidAptHookLine {
        line: String,
    },
    FailedToAccessDebianService {
        kind: ureq::ErrorKind,
        response: Option<String>,
    },
    MalformedResponseFromDebianService,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NoAptHookInfoFdVariable => {
                write!(f, "No APT_HOOK_INFO_FD environment variable")
            }
            AppError::InvalidAptHookInfoVariable => {
                write!(f, "Invalid APT_HOOK_INFO_FD (NaN)")
            }
            AppError::CantReadFromAptHook => {
                write!(f, "Cannot read from descriptor APT_HOOK_INFO_FD")
            }
            AppError::NoNewlineBetweenAptHookParts => {
                write!(f, "No newline between contents of APT_HOOK_INFO_FD")
            }
            AppError::InvalidAptHookLine { line } => {
                write!(f, "Invalid line in APT_HOOK_INFO_FD: {line}")
            }
            AppError::FailedToAccessDebianService { kind, response } => {
                write!(
                    f,
                    "Failed to access Debian server: {} ({})",
                    kind,
                    response.as_deref().unwrap_or("<empty response>")
                )
            }
            AppError::MalformedResponseFromDebianService => {
                write!(f, "Malformed response from the Debian server")
            }
        }
    }
}
