//! JSON serialization of [`ProblemDetails`], asserted against expected bodies.
//!
//! Every case here has a same-named counterpart in `xml_serialization.rs`, both
//! built from the same fixture in `common::cases`.
#![cfg(feature = "json")]

mod common;

use common::assert::{assert_json_body, assert_json_members_unique};
use common::cases;
use serde_json::json;

// --- structure -------------------------------------------------------------

#[test]
fn empty() {
    assert_json_body(cases::empty(), json!({}));
}

#[test]
fn all_members() {
    assert_json_body(
        cases::all_members(),
        json!({
            "type": "test:type",
            "status": 404,
            "title": "Test Title",
            "detail": "Test Detail",
            "instance": "test:instance",
        }),
    );
}

/// RFC 9457 §3, verbatim.
#[test]
fn rfc_out_of_credit() {
    assert_json_body(
        cases::rfc_out_of_credit(),
        json!({
            "type": "https://example.com/probs/out-of-credit",
            "title": "You do not have enough credit.",
            "detail": "Your current balance is 30, but that costs 50.",
            "instance": "/account/12345/msgs/abc",
            "balance": 30,
            "accounts": ["/account/12345", "/account/67890"],
        }),
    );
}

/// `status` is omitted entirely rather than written as `null` — see finding #7
/// for why that is awkward next to a 500 response.
#[test]
fn status_omitted() {
    assert_json_body(
        cases::status_omitted(),
        json!({ "title": "Something went wrong" }),
    );
}

#[test]
fn type_explicit_about_blank() {
    assert_json_body(
        cases::type_explicit_about_blank(),
        json!({ "type": "about:blank", "status": 400 }),
    );
}

#[test]
fn extensions_only() {
    assert_json_body(
        cases::extensions_only(),
        json!({
            "text": "some text",
            "count": 42,
            "signed": -7,
            "flag": true,
            "ratio": 1.5,
        }),
    );
}

// --- values ----------------------------------------------------------------

#[test]
fn markup_in_text() {
    assert_json_body(
        cases::markup_in_text(),
        json!({
            "title": r#"Tom & Jerry's "<b>bold</b>" plan"#,
            "detail": "5 < 6 && 7 > 6",
        }),
    );
}

#[test]
fn non_ascii_text() {
    assert_json_body(
        cases::non_ascii_text(),
        json!({
            "title": "Ünïcödé — 日本語 🚀",
            "detail": "combining: e\u{0301}, precomposed: é",
        }),
    );
}

#[test]
fn control_whitespace_in_detail() {
    assert_json_body(
        cases::control_whitespace_in_detail(),
        json!({ "detail": "line one\nline two\tindented" }),
    );
}

#[test]
fn empty_title() {
    assert_json_body(cases::empty_title(), json!({ "title": "" }));
}

#[test]
fn padded_title() {
    assert_json_body(cases::padded_title(), json!({ "title": "  padded  " }));
}

#[test]
fn uris_with_query_and_escapes() {
    assert_json_body(
        cases::uris_with_query_and_escapes(),
        json!({
            "type": "https://example.com/probs/out-of-credit?since=2024-01-01&until=2024-12-31",
            "instance": "/accounts/12345/msgs/a%20b",
        }),
    );
}

/// RFC 9457 §3.1.1 makes `type` a *URI reference*, which includes a fragment,
/// and §6.1 suggests registering types under a prefix that ends in `#`.
#[test]
#[ignore = "finding #22: http::Uri drops the fragment, so registered type URIs cannot be represented"]
fn iana_registered_type() {
    assert_json_body(
        cases::iana_registered_type(),
        json!({ "type": "https://iana.org/assignments/http-problem-types#out-of-credit" }),
    );
}

/// A `urn:` type is an ordinary URI reference; `http::Uri` rejects it outright.
#[test]
#[ignore = "finding #23: http::Uri rejects urn: URIs with InvalidAuthority"]
fn urn_type() {
    assert_json_body(
        cases::urn_type(),
        json!({ "type": "urn:problem-type:out-of-credit" }),
    );
}

#[test]
fn status_lowest() {
    assert_json_body(cases::status_lowest(), json!({ "status": 100 }));
}

#[test]
fn status_highest() {
    assert_json_body(cases::status_highest(), json!({ "status": 599 }));
}

// --- extensions ------------------------------------------------------------

#[test]
fn ext_flat() {
    assert_json_body(
        cases::ext_flat(),
        json!({
            "status": 400,
            "text": "some text",
            "count": 42,
            "signed": -7,
            "flag": true,
            "ratio": 1.5,
        }),
    );
}

#[test]
fn ext_vec() {
    assert_json_body(
        cases::ext_vec(),
        json!({ "accounts": ["/account/12345", "/account/67890"] }),
    );
}

#[test]
fn ext_nested() {
    assert_json_body(
        cases::ext_nested(),
        json!({
            "outer": "top",
            "inner": { "number": 1, "text": "deep" },
        }),
    );
}

#[test]
fn ext_option_none() {
    assert_json_body(
        cases::ext_option_none(),
        json!({ "present": 1, "absent": null }),
    );
}

#[test]
fn ext_map() {
    assert_json_body(
        cases::ext_map(),
        json!({ "title": "Dynamic", "alpha": "first", "beta": 2 }),
    );
}

/// RFC 8259 §4 says member names SHOULD be unique, because a consumer's
/// handling of duplicates is unpredictable. `serde(flatten)` writes both.
#[test]
#[ignore = "finding #25: a colliding extension produces a duplicate member name"]
fn ext_collides_with_member() {
    assert_json_members_unique(cases::ext_collides_with_member());
}
