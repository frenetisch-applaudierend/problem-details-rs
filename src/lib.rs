mod problem_details;
mod problem_type;

pub use problem_details::ProblemDetails;
pub use problem_type::ProblemType;

// Serde related extensions for http
#[cfg(feature = "serde")]
mod serde;

// Re-export often used http structs
pub use http::StatusCode;
pub use http::Uri;
