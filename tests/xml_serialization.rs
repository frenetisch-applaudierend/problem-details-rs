//! XML serialization of [`ProblemDetails`], asserted against expected bodies.
//!
//! Every case here has a same-named counterpart in `json_serialization.rs`,
//! both built from the same fixture in `common::cases`.
//!
//! Expected bodies are written the way RFC 9457 Appendix B prescribes, and may
//! be pretty-printed — the comparison is whitespace- and entity-tolerant, see
//! `common::xml_canon`. Cases the crate does not satisfy yet are `#[ignore]`d
//! with the finding they are waiting on; `cargo test -- --ignored` lists them.
#![cfg(feature = "xml")]

mod common;

use common::assert::{RFC_NAMESPACE, assert_xml_body, assert_xml_members_unique};
use common::cases;

// --- structure -------------------------------------------------------------

#[test]
fn empty() {
    assert_xml_body(cases::empty(), r#"<problem xmlns="urn:ietf:rfc:7807"/>"#);
}

#[test]
fn all_members() {
    assert_xml_body(
        cases::all_members(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <type>test:type</type>
            <status>404</status>
            <title>Test Title</title>
            <detail>Test Detail</detail>
            <instance>test:instance</instance>
        </problem>
        "#,
    );
}

/// RFC 9457 Appendix B, verbatim.
#[test]
fn rfc_out_of_credit() {
    assert_xml_body(
        cases::rfc_out_of_credit(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <type>https://example.com/probs/out-of-credit</type>
            <title>You do not have enough credit.</title>
            <detail>Your current balance is 30, but that costs 50.</detail>
            <instance>/account/12345/msgs/abc</instance>
            <balance>30</balance>
            <accounts>
                <i>/account/12345</i>
                <i>/account/67890</i>
            </accounts>
        </problem>
        "#,
    );
}

#[test]
fn status_omitted() {
    assert_xml_body(
        cases::status_omitted(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <title>Something went wrong</title>
        </problem>
        "#,
    );
}

#[test]
fn type_explicit_about_blank() {
    assert_xml_body(
        cases::type_explicit_about_blank(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <type>about:blank</type>
            <status>400</status>
        </problem>
        "#,
    );
}

#[test]
fn extensions_only() {
    assert_xml_body(
        cases::extensions_only(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <text>some text</text>
            <count>42</count>
            <signed>-7</signed>
            <flag>true</flag>
            <ratio>1.5</ratio>
        </problem>
        "#,
    );
}

/// RFC 9457 Appendix B mandates the namespace on the root element.
///
/// Every other case in this file asserts it too, since the comparison resolves
/// namespaces; this one states it on its own so a regression names itself.
#[test]
fn root_declares_rfc_namespace() {
    use common::xml_canon::{Event, canonicalize};

    let body = problem_details::XmlProblemDetails::from(cases::all_members())
        .to_body_string()
        .expect("could not serialize to XML");

    let events = canonicalize(&body).expect("body is not well-formed");

    let Some(Event::Start { name, .. }) = events.first() else {
        panic!("body has no root element:\n{body}");
    };

    assert_eq!(name.local, "problem");
    assert_eq!(name.ns.as_deref(), Some(RFC_NAMESPACE), "in\n{body}");
}

// --- values ----------------------------------------------------------------

#[test]
fn markup_in_text() {
    assert_xml_body(
        cases::markup_in_text(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <title>Tom &amp; Jerry&apos;s &quot;&lt;b&gt;bold&lt;/b&gt;&quot; plan</title>
            <detail>5 &lt; 6 &amp;&amp; 7 &gt; 6</detail>
        </problem>
        "#,
    );
}

/// The combining mark is written as a character reference so that it stays
/// visibly distinct from the precomposed `é` next to it — the canonicalizer
/// resolves the reference, so this also covers `&#…;` in element content.
#[test]
fn non_ascii_text() {
    assert_xml_body(
        cases::non_ascii_text(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <title>Ünïcödé — 日本語 🚀</title>
            <detail>combining: e&#x301;, precomposed: é</detail>
        </problem>
        "#,
    );
}

/// The newline and tab are element content, so they survive verbatim rather
/// than being escaped the way JSON escapes them.
#[test]
fn control_whitespace_in_detail() {
    assert_xml_body(
        cases::control_whitespace_in_detail(),
        "<problem xmlns=\"urn:ietf:rfc:7807\">\
         <detail>line one\nline two\tindented</detail>\
         </problem>",
    );
}

#[test]
fn empty_title() {
    assert_xml_body(
        cases::empty_title(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <title/>
        </problem>
        "#,
    );
}

/// Written on one line: the padding is element content, and the comparison
/// keeps a leaf element's text exactly as it is.
#[test]
fn padded_title() {
    assert_xml_body(
        cases::padded_title(),
        r#"<problem xmlns="urn:ietf:rfc:7807"><title>  padded  </title></problem>"#,
    );
}

#[test]
fn uris_with_query_and_escapes() {
    assert_xml_body(
        cases::uris_with_query_and_escapes(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <type>https://example.com/probs/out-of-credit?since=2024-01-01&amp;until=2024-12-31</type>
            <instance>/accounts/12345/msgs/a%20b</instance>
        </problem>
        "#,
    );
}

/// RFC 9457 §3.1.1 makes `type` a *URI reference*, which includes a fragment,
/// and §6.1 suggests registering types under a prefix that ends in `#`.
#[test]
#[ignore = "finding #22: http::Uri drops the fragment, so registered type URIs cannot be represented"]
fn iana_registered_type() {
    assert_xml_body(
        cases::iana_registered_type(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <type>https://iana.org/assignments/http-problem-types#out-of-credit</type>
        </problem>
        "#,
    );
}

/// A `urn:` type is an ordinary URI reference; `http::Uri` rejects it outright.
#[test]
#[ignore = "finding #23: http::Uri rejects urn: URIs with InvalidAuthority"]
fn urn_type() {
    assert_xml_body(
        cases::urn_type(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <type>urn:problem-type:out-of-credit</type>
        </problem>
        "#,
    );
}

#[test]
fn status_lowest() {
    assert_xml_body(
        cases::status_lowest(),
        r#"<problem xmlns="urn:ietf:rfc:7807"><status>100</status></problem>"#,
    );
}

#[test]
fn status_highest() {
    assert_xml_body(
        cases::status_highest(),
        r#"<problem xmlns="urn:ietf:rfc:7807"><status>599</status></problem>"#,
    );
}

// --- extensions ------------------------------------------------------------

#[test]
fn ext_flat() {
    assert_xml_body(
        cases::ext_flat(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <status>400</status>
            <text>some text</text>
            <count>42</count>
            <signed>-7</signed>
            <flag>true</flag>
            <ratio>1.5</ratio>
        </problem>
        "#,
    );
}

#[test]
fn ext_vec() {
    assert_xml_body(
        cases::ext_vec(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <accounts>
                <i>/account/12345</i>
                <i>/account/67890</i>
            </accounts>
        </problem>
        "#,
    );
}

#[test]
fn ext_nested() {
    assert_xml_body(
        cases::ext_nested(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <outer>top</outer>
            <inner>
                <number>1</number>
                <text>deep</text>
            </inner>
        </problem>
        "#,
    );
}

/// RFC 9457 Appendix B has no XML representation for null, so the only
/// faithful option is to leave the member out: `<absent/>` is a positive claim
/// that the value is the empty string, and reads back as `Some("")`.
#[test]
#[ignore = "finding #24: a None extension is written as an empty element, not omitted"]
fn ext_option_none() {
    assert_xml_body(
        cases::ext_option_none(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <present>1</present>
        </problem>
        "#,
    );
}

#[cfg(feature = "json")]
#[test]
fn ext_map() {
    assert_xml_body(
        cases::ext_map(),
        r#"
        <problem xmlns="urn:ietf:rfc:7807">
            <title>Dynamic</title>
            <alpha>first</alpha>
            <beta>2</beta>
        </problem>
        "#,
    );
}

/// Two `<title>` children make `<problem>` an object with a repeated member
/// name — and, under Appendix B's rules, indistinguishable from an array that
/// forgot its `<i>` wrappers.
#[test]
#[ignore = "finding #25: a colliding extension produces a duplicate member name"]
fn ext_collides_with_member() {
    assert_xml_members_unique(cases::ext_collides_with_member());
}
