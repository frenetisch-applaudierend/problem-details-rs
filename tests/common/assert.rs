//! Assertions on the serialized body of a [`ProblemDetails`].
//!
//! Both go through the public `to_body_string()` of the wrapper types, because
//! that — not the bare serde impl — is what a framework integration puts on the
//! wire, and it is where the XML root element, prolog and namespace live.

#[allow(unused_imports)]
use problem_details::ProblemDetails;

/// The namespace RFC 9457 Appendix B mandates on the root element.
///
/// Expected bodies are written the way the RFC prescribes, with
/// `xmlns="urn:ietf:rfc:7807"` on `<problem>`, and [`assert_xml_body`] compares
/// namespaces, so every case asserts it — `root_declares_rfc_namespace` only
/// states it in isolation.
#[cfg(feature = "xml")]
pub const RFC_NAMESPACE: &str = "urn:ietf:rfc:7807";

/// Assert that `details` serializes to JSON equal to `expected`.
///
/// Both sides are compared as parsed [`serde_json::Value`]s, so key order and
/// whitespace are irrelevant while presence, absence and value type are not.
#[cfg(feature = "json")]
pub fn assert_json_body<Ext>(details: ProblemDetails<Ext>, expected: serde_json::Value)
where
    Ext: serde::Serialize,
{
    let body = json_body(details);

    // Checked on every case, not just the one that violates it: a
    // `serde_json::Value` comparison is blind to duplicate member names.
    check_json_members_unique(&body);

    let actual: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("body is not valid JSON: {err}\nbody: {body}"));

    assert_eq!(
        actual, expected,
        "\nunexpected JSON body\n  actual body: {body}\n"
    );
}

/// Assert that the serialized JSON object has no repeated member name.
///
/// RFC 8259 §4: *"The names within an object SHOULD be unique"*, because
/// *"when the names within an object are not unique, the behavior of software
/// that receives such an object is unpredictable"*.
///
/// A plain [`serde_json::Value`] comparison cannot see this — parsing collapses
/// duplicates — so the member names are collected off the raw token stream.
#[cfg(feature = "json")]
pub fn assert_json_members_unique<Ext>(details: ProblemDetails<Ext>)
where
    Ext: serde::Serialize,
{
    check_json_members_unique(&json_body(details));
}

#[cfg(feature = "json")]
fn check_json_members_unique(body: &str) {
    let mut seen = std::collections::BTreeSet::new();
    let duplicates = json_member_names(body)
        .into_iter()
        .filter(|name| !seen.insert(name.clone()))
        .collect::<Vec<_>>();

    assert!(
        duplicates.is_empty(),
        "JSON object has repeated member name(s) {duplicates:?}\n  body: {body}"
    );
}

/// The top-level member names of a JSON object, in order and *with* duplicates.
#[cfg(feature = "json")]
fn json_member_names(body: &str) -> Vec<String> {
    struct Collector;

    impl<'de> serde::de::Visitor<'de> for Collector {
        type Value = Vec<String>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a JSON object")
        }

        fn visit_map<A: serde::de::MapAccess<'de>>(
            self,
            mut map: A,
        ) -> Result<Self::Value, A::Error> {
            let mut names = Vec::new();
            while let Some(name) = map.next_key::<String>()? {
                map.next_value::<serde::de::IgnoredAny>()?;
                names.push(name);
            }
            Ok(names)
        }
    }

    let mut deserializer = serde_json::Deserializer::from_str(body);
    serde::Deserializer::deserialize_map(&mut deserializer, Collector)
        .unwrap_or_else(|err| panic!("body is not a JSON object: {err}\n  body: {body}"))
}

#[cfg(feature = "json")]
fn json_body<Ext>(details: ProblemDetails<Ext>) -> String
where
    Ext: serde::Serialize,
{
    problem_details::JsonProblemDetails::from(details)
        .to_body_string()
        .expect("could not serialize to JSON")
}

/// Assert that no two children of `<problem>` share an element name.
///
/// The XML counterpart of [`assert_json_members_unique`]. RFC 9457 Appendix B
/// reads an element with children as an object, so two same-named children are
/// two members with the same name — the same ambiguity RFC 8259 §4 warns about,
/// and indistinguishable from the `<i>`-less array of finding #5.
#[cfg(feature = "xml")]
pub fn assert_xml_members_unique<Ext>(details: ProblemDetails<Ext>)
where
    Ext: serde::Serialize,
{
    let body = xml_body(details);
    let events =
        super::xml_canon::canonicalize(&body).unwrap_or_else(|err| panic!("actual body: {err}"));

    check_xml_members_unique(&events, &body);
}

#[cfg(feature = "xml")]
fn check_xml_members_unique(events: &[super::xml_canon::Event], body: &str) {
    use super::xml_canon::Event;

    let mut depth = 0usize;
    let mut seen = std::collections::BTreeSet::new();
    let mut duplicates = Vec::new();

    for event in events {
        match event {
            Event::Start { name, .. } => {
                // Depth 0 is `<problem>` itself; its members sit at depth 1.
                if depth == 1 && !seen.insert(name.to_string()) {
                    duplicates.push(name.to_string());
                }
                depth += 1;
            }
            Event::End { .. } => depth -= 1,
            Event::Text(_) => {}
        }
    }

    assert!(
        duplicates.is_empty(),
        "<problem> has repeated child element name(s) {duplicates:?}\n  body: {body}"
    );
}

/// Assert that `details` serializes to XML equivalent to `expected`.
///
/// Equivalence is up to the normalizations documented on
/// [`crate::common::xml_canon`]; in particular `expected` may be
/// pretty-printed and may omit the XML declaration.
#[cfg(feature = "xml")]
pub fn assert_xml_body<Ext>(details: ProblemDetails<Ext>, expected: &str)
where
    Ext: serde::Serialize,
{
    use super::xml_canon;

    let body = xml_body(details);

    xml_canon::assert_declaration(&body);

    let actual = xml_canon::canonicalize(&body).unwrap_or_else(|err| panic!("actual body: {err}"));

    // Checked on every case, not just the one that violates it.
    check_xml_members_unique(&actual, &body);

    let expected_events =
        xml_canon::canonicalize(expected).unwrap_or_else(|err| panic!("expected body: {err}"));

    if actual == expected_events {
        return;
    }

    let index = actual
        .iter()
        .zip(&expected_events)
        .position(|(actual, expected)| actual != expected)
        .unwrap_or_else(|| actual.len().min(expected_events.len()));

    panic!(
        "\nunexpected XML body, first difference at canonical event #{index}\n  \
         expected: {:?}\n  \
         actual:   {:?}\n\n\
         --- expected ---\n{expected}\n\n\
         --- actual ---\n{body}\n",
        expected_events.get(index),
        actual.get(index),
    );
}

#[cfg(feature = "xml")]
fn xml_body<Ext>(details: ProblemDetails<Ext>) -> String
where
    Ext: serde::Serialize,
{
    problem_details::XmlProblemDetails::from(details)
        .to_body_string()
        .expect("could not serialize to XML")
}
