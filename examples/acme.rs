//! Maps the error types of RFC 8555 (ACME) to `ProblemType` using only std conversion traits.
use http::StatusCode;
use problem_details::{ProblemDetails, ProblemType};

/// The problem types a client of an ACME server needs to distinguish.
#[derive(Debug, Clone, PartialEq, Eq)]
enum AcmeError {
    BadNonce,
    Malformed,
    RateLimited,
    /// Any other problem type as sent, or `None` if the problem had no type at all
    Other(Option<String>),
}

impl AcmeError {
    fn urn(&self) -> Option<&str> {
        match self {
            Self::BadNonce => Some("urn:ietf:params:acme:error:badNonce"),
            Self::Malformed => Some("urn:ietf:params:acme:error:malformed"),
            Self::RateLimited => Some("urn:ietf:params:acme:error:rateLimited"),
            Self::Other(urn) => urn.as_deref(),
        }
    }
}

// Sending: allows `ProblemDetails::with_type(AcmeError::BadNonce)`
impl From<AcmeError> for ProblemType {
    fn from(error: AcmeError) -> Self {
        // on the sending side, we would likely not have AcmeError::Other so this would be simply error.urn().map(ProblemType::unchecked)
        error
            .urn()
            .map_or_else(ProblemType::default, ProblemType::unchecked)
    }
}

// Receiving: unknown or missing types end up in `Other`
impl From<Option<ProblemType>> for AcmeError {
    fn from(value: Option<ProblemType>) -> Self {
        match value.as_ref().map(ProblemType::as_str) {
            Some("urn:ietf:params:acme:error:badNonce") => Self::BadNonce,
            Some("urn:ietf:params:acme:error:malformed") => Self::Malformed,
            Some("urn:ietf:params:acme:error:rateLimited") => Self::RateLimited,
            other => Self::Other(other.map(str::to_string)),
        }
    }
}

fn main() {
    // Server side: build and serialize a problem
    let problem = ProblemDetails::from_status_code(StatusCode::BAD_REQUEST)
        .with_type(AcmeError::BadNonce)
        .with_detail("JWS has an invalid anti-replay nonce");

    let json = serde_json::to_string_pretty(&problem).unwrap();
    println!("{json}");

    // Client side: deserialize leniently and match on the known types
    let received: ProblemDetails = serde_json::from_str(&json).unwrap();

    match AcmeError::from(received.r#type) {
        AcmeError::BadNonce => println!("retrying with a fresh nonce"),
        AcmeError::Other(Some(urn)) => println!("unknown problem type: {urn}"),
        AcmeError::Other(None) => println!("no problem type (about:blank)"),
        other => println!("known error: {other:?}"),
    }
}
