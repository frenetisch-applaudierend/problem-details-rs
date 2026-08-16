//! The problem details instances under test.
//!
//! Shared so that the JSON and the XML suite exercise the *same* values, and a
//! case can be looked up by the same name in both.

use http::{StatusCode, Uri};
use problem_details::ProblemDetails;

// --- structure -------------------------------------------------------------

/// Nothing set at all — the `ProblemDetails::default()` shape.
pub fn empty() -> ProblemDetails {
    ProblemDetails::new()
}

/// Every core member set, no extensions.
pub fn all_members() -> ProblemDetails {
    ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_status(StatusCode::NOT_FOUND)
        .with_title("Test Title")
        .with_detail("Test Detail")
        .with_instance(Uri::from_static("test:instance"))
}

/// The worked example from RFC 9457 §3, extensions and all.
pub fn rfc_out_of_credit() -> ProblemDetails<OutOfCredit> {
    ProblemDetails::new()
        .with_type(Uri::from_static("https://example.com/probs/out-of-credit"))
        .with_title("You do not have enough credit.")
        .with_detail("Your current balance is 30, but that costs 50.")
        .with_instance(Uri::from_static("/account/12345/msgs/abc"))
        .with_extensions(OutOfCredit {
            balance: 30,
            accounts: vec!["/account/12345".to_string(), "/account/67890".to_string()],
        })
}

#[derive(Debug, serde::Serialize)]
pub struct OutOfCredit {
    pub balance: u32,
    pub accounts: Vec<String>,
}

/// No `status` member, which the integrations turn into a 500 response whose
/// body says nothing about the status. Pins the behaviour finding #7 is about.
pub fn status_omitted() -> ProblemDetails {
    ProblemDetails::new().with_title("Something went wrong")
}

/// `about:blank` written out, rather than left implicit by omitting `type`.
pub fn type_explicit_about_blank() -> ProblemDetails {
    ProblemDetails::new()
        .with_type(Uri::from_static("about:blank"))
        .with_status(StatusCode::BAD_REQUEST)
}

/// Extensions without a single core member.
pub fn extensions_only() -> ProblemDetails<Flat> {
    ProblemDetails::new().with_extensions(flat_extensions())
}

// --- values ----------------------------------------------------------------

/// Characters that have to be escaped in XML, and quotes that have to be
/// escaped in JSON.
pub fn markup_in_text() -> ProblemDetails {
    ProblemDetails::new()
        .with_title(r#"Tom & Jerry's "<b>bold</b>" plan"#)
        .with_detail("5 < 6 && 7 > 6")
}

/// Non-ASCII text: accented Latin, CJK, an emoji, and a combining mark that
/// must not be normalized away.
pub fn non_ascii_text() -> ProblemDetails {
    ProblemDetails::new()
        .with_title("Ünïcödé — 日本語 🚀")
        .with_detail("combining: e\u{0301}, precomposed: é")
}

/// A newline and a tab inside `detail`, which JSON escapes and XML does not.
pub fn control_whitespace_in_detail() -> ProblemDetails {
    ProblemDetails::new().with_detail("line one\nline two\tindented")
}

/// An empty string is a different thing from an absent member.
pub fn empty_title() -> ProblemDetails {
    ProblemDetails::new().with_title("")
}

/// Leading and trailing whitespace inside a member is significant, and must
/// survive a serialize/compare round even though the XML comparison is
/// otherwise whitespace-tolerant.
pub fn padded_title() -> ProblemDetails {
    ProblemDetails::new().with_title("  padded  ")
}

/// A `type` with a query string containing `&` (escaped in XML, not in JSON)
/// and a percent-encoded relative `instance`.
pub fn uris_with_query_and_escapes() -> ProblemDetails {
    ProblemDetails::new()
        .with_type(Uri::from_static(
            "https://example.com/probs/out-of-credit?since=2024-01-01&until=2024-12-31",
        ))
        .with_instance(Uri::from_static("/accounts/12345/msgs/a%20b"))
}

/// A problem type under the prefix RFC 9457 §6.1 suggests for registrations,
/// `https://iana.org/assignments/http-problem-types#<name>`. A fragment is not
/// an exotic case here — it is the shape the RFC itself recommends.
///
/// `http::Uri` models an HTTP *request target*, where the fragment is by
/// definition never transmitted, so it discards the fragment while parsing.
/// The field type therefore cannot hold one at all, and every type registered
/// under that prefix collapses to the same string.
pub fn iana_registered_type() -> ProblemDetails {
    ProblemDetails::new().with_type(
        "https://iana.org/assignments/http-problem-types#out-of-credit"
            .parse::<Uri>()
            .expect("a URI reference with a fragment"),
    )
}

/// A `urn:` problem type — a perfectly ordinary URI reference, and the usual
/// choice for APIs that do not want a dereferenceable type.
///
/// `http::Uri` rejects every multi-segment URN with `InvalidAuthority`
/// (`urn:example` parses, `urn:example:type` does not), so this panics on
/// construction rather than producing a wrong body.
pub fn urn_type() -> ProblemDetails {
    ProblemDetails::new().with_type(
        "urn:problem-type:out-of-credit"
            .parse::<Uri>()
            .expect("http::Uri cannot parse urn: URIs"),
    )
}

/// The lowest status code RFC 9110 §15 defines.
///
/// Note this is *not* the lowest `http::StatusCode` accepts — that type takes
/// anything in `100..=999`, so a caller can hand the crate a `600` that no
/// status class covers. Out of scope here: the value comes from the caller.
pub fn status_lowest() -> ProblemDetails {
    ProblemDetails::new().with_status(StatusCode::from_u16(100).unwrap())
}

/// The highest status code RFC 9110 §15 defines — the top of the 5xx class.
pub fn status_highest() -> ProblemDetails {
    ProblemDetails::new().with_status(StatusCode::from_u16(599).unwrap())
}

// --- extensions ------------------------------------------------------------

/// A flat extension struct covering the scalar types.
pub fn ext_flat() -> ProblemDetails<Flat> {
    ProblemDetails::new()
        .with_status(StatusCode::BAD_REQUEST)
        .with_extensions(flat_extensions())
}

fn flat_extensions() -> Flat {
    Flat {
        text: "some text".to_string(),
        count: 42,
        signed: -7,
        flag: true,
        ratio: 1.5,
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Flat {
    pub text: String,
    pub count: u32,
    pub signed: i64,
    pub flag: bool,
    pub ratio: f64,
}

/// A sequence extension. RFC 9457 Appendix B represents these as a container
/// element with one `<i>` per item; see finding #5.
pub fn ext_vec() -> ProblemDetails<WithVec> {
    ProblemDetails::new().with_extensions(WithVec {
        accounts: vec!["/account/12345".to_string(), "/account/67890".to_string()],
    })
}

#[derive(Debug, serde::Serialize)]
pub struct WithVec {
    pub accounts: Vec<String>,
}

/// An extension holding another struct.
pub fn ext_nested() -> ProblemDetails<Nested> {
    ProblemDetails::new().with_extensions(Nested {
        outer: "top".to_string(),
        inner: Inner {
            number: 1,
            text: "deep".to_string(),
        },
    })
}

#[derive(Debug, serde::Serialize)]
pub struct Nested {
    pub outer: String,
    pub inner: Inner,
}

#[derive(Debug, serde::Serialize)]
pub struct Inner {
    pub number: u32,
    pub text: String,
}

/// An extension field that is `None`, without `skip_serializing_if`.
///
/// Whether a `None` is written at all is the extension author's call, not the
/// crate's; this fixture deliberately omits `skip_serializing_if` to cover the
/// case where they do not make that call.
///
/// JSON writes `null`, which is legal and round-trips back to `None`. XML has
/// no representation for null — see finding #24.
pub fn ext_option_none() -> ProblemDetails<WithOptions> {
    ProblemDetails::new().with_extensions(WithOptions {
        present: Some(1),
        absent: None,
    })
}

#[derive(Debug, serde::Serialize)]
pub struct WithOptions {
    pub present: Option<u32>,
    pub absent: Option<u32>,
}

/// Dynamic extensions. A `BTreeMap` rather than a `HashMap` so the member
/// order is deterministic — XML comparison is order-sensitive.
#[cfg(feature = "json")]
pub fn ext_map() -> ProblemDetails<std::collections::BTreeMap<String, serde_json::Value>> {
    let extensions = [
        ("alpha".to_string(), serde_json::json!("first")),
        ("beta".to_string(), serde_json::json!(2)),
    ]
    .into_iter()
    .collect::<std::collections::BTreeMap<_, _>>();

    ProblemDetails::new()
        .with_title("Dynamic")
        .with_extensions(extensions)
}

/// An extension whose field name collides with a core member.
///
/// `serde(flatten)` does not deduplicate, so both are written and the result
/// has a duplicate member name — which RFC 8259 §4 says makes consumer
/// behaviour unpredictable. See finding #25.
pub fn ext_collides_with_member() -> ProblemDetails<Collides> {
    ProblemDetails::new()
        .with_title("core title")
        .with_extensions(Collides {
            title: "extension title".to_string(),
        })
}

#[derive(Debug, serde::Serialize)]
pub struct Collides {
    pub title: String,
}
