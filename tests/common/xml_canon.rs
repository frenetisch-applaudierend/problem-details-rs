//! Canonicalization of XML documents, so that expected bodies can be written
//! in a readable, pretty-printed form without the tests becoming assertions
//! about the serializer's whitespace habits.
//!
//! The rules, and the reasoning behind each:
//!
//! | Aspect | Treatment |
//! |---|---|
//! | element order | significant — serde field order is deterministic |
//! | namespaces | compared *resolved*, so `xmlns="…"` and `<p:x xmlns:p="…">` are equal |
//! | `<a/>` vs `<a></a>` | equal |
//! | whitespace between elements | ignored |
//! | text inside a leaf element | kept verbatim, *not* trimmed |
//! | entities | decoded before comparison, so `&amp;` and `&#38;` are equal |
//! | attributes | sorted by resolved name, values decoded |
//! | XML declaration | checked separately by [`assert_declaration`] |
//!
//! Text is only discarded when the element has element children — this schema
//! has no mixed content, so a whitespace-only text node there is by definition
//! pretty-printing. A leaf element keeps its text exactly, which is what lets
//! a title of `" padded "` fail instead of silently passing as `"padded"`.

use quick_xml::escape::resolve_predefined_entity;
use quick_xml::events::Event as XmlEvent;
use quick_xml::name::ResolveResult;
use quick_xml::{NsReader, Reader, XmlVersion};

/// The bodies under test are XML 1.0; quick-xml needs this to pick the right
/// end-of-line normalization rules.
const VERSION: XmlVersion = XmlVersion::Explicit1_0;

/// An element or attribute name, with its prefix already resolved to a
/// namespace URI (`None` meaning "in no namespace").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub ns: Option<String>,
    pub local: String,
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.ns {
            Some(ns) => write!(f, "{{{ns}}}{}", self.local),
            None => write!(f, "{}", self.local),
        }
    }
}

/// A single event of the canonical form of a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Start {
        name: Name,
        attrs: Vec<(Name, String)>,
    },
    End {
        name: Name,
    },
    Text(String),
}

/// Parse `xml` into its canonical event sequence.
pub fn canonicalize(xml: &str) -> Result<Vec<Event>, String> {
    let roots = parse(xml)?;

    let mut events = Vec::new();
    for root in &roots {
        flatten(root, &mut events);
    }

    Ok(events)
}

/// Remove all namespaces from a canonical event sequence.
///
/// Used to downgrade the expected side while [`crate::common::assert::EXPECTED_NAMESPACE`]
/// is `None` — see the note there.
pub fn strip_namespaces(events: &mut [Event]) {
    for event in events {
        match event {
            Event::Start { name, attrs } => {
                name.ns = None;
                for (attr, _) in attrs {
                    attr.ns = None;
                }
            }
            Event::End { name } => name.ns = None,
            Event::Text(_) => {}
        }
    }
}

/// Assert that `xml` starts with an XML declaration of version 1.0 and,
/// if an encoding is given, UTF-8.
///
/// Kept out of [`canonicalize`] so that quoting and spacing inside the
/// declaration stay irrelevant.
pub fn assert_declaration(xml: &str) {
    let mut reader = Reader::from_str(xml);

    match reader.read_event() {
        Ok(XmlEvent::Decl(decl)) => {
            let version = decl.version().expect("declaration has no version");
            let version = String::from_utf8_lossy(&version).into_owned();
            assert_eq!(version, "1.0", "unexpected XML version in\n{xml}");

            if let Some(encoding) = decl.encoding() {
                let encoding = encoding.expect("declaration has a malformed encoding");
                let encoding = String::from_utf8_lossy(&encoding).into_owned();
                assert!(
                    encoding.eq_ignore_ascii_case("utf-8"),
                    "unexpected XML encoding {encoding:?} in\n{xml}"
                );
            }
        }
        other => panic!("expected an XML declaration, got {other:?} in\n{xml}"),
    }
}

struct Node {
    name: Name,
    attrs: Vec<(Name, String)>,
    children: Vec<Child>,
}

enum Child {
    Elem(Node),
    Text(String),
}

fn parse(xml: &str) -> Result<Vec<Node>, String> {
    let mut reader = NsReader::from_str(xml);
    reader.config_mut().expand_empty_elements = true;
    reader.config_mut().check_end_names = true;

    let mut stack: Vec<Node> = Vec::new();
    let mut roots: Vec<Node> = Vec::new();

    loop {
        let (ns, event) = reader
            .read_resolved_event()
            .map_err(|err| format!("not well-formed XML: {err}\n{xml}"))?;

        match event {
            XmlEvent::Start(start) => {
                let name = Name {
                    ns: resolved(ns),
                    local: to_string(start.local_name().as_ref()),
                };

                let mut attrs = Vec::new();
                for attr in start.attributes() {
                    let attr = attr.map_err(|err| format!("bad attribute: {err}\n{xml}"))?;

                    // Namespace declarations are syntax, not data — dropping them
                    // is what makes prefixed and default namespaces compare equal.
                    if is_namespace_declaration(attr.key.as_ref()) {
                        continue;
                    }

                    let (attr_ns, local) = reader.resolver_mut().resolve_attribute(attr.key);
                    let attr_ns = resolved(attr_ns);
                    let value = attr
                        .normalized_value(VERSION)
                        .map_err(|err| format!("bad attribute value: {err}\n{xml}"))?;

                    attrs.push((
                        Name {
                            ns: attr_ns,
                            local: to_string(local.as_ref()),
                        },
                        value.into_owned(),
                    ));
                }
                attrs.sort_by(|(a, _), (b, _)| (&a.ns, &a.local).cmp(&(&b.ns, &b.local)));

                stack.push(Node {
                    name,
                    attrs,
                    children: Vec::new(),
                });
            }

            XmlEvent::End(_) => {
                let node = stack
                    .pop()
                    .ok_or_else(|| format!("unbalanced tags\n{xml}"))?;
                match stack.last_mut() {
                    Some(parent) => parent.children.push(Child::Elem(node)),
                    None => roots.push(node),
                }
            }

            XmlEvent::Text(text) => {
                let text = text
                    .xml_content(VERSION)
                    .map_err(|err| format!("bad text content: {err}\n{xml}"))?;
                // Text outside the root element is only ever whitespace.
                if let Some(node) = stack.last_mut() {
                    node.children.push(Child::Text(text.into_owned()));
                }
            }

            XmlEvent::CData(cdata) => {
                if let Some(node) = stack.last_mut() {
                    node.children.push(Child::Text(to_string(&cdata)));
                }
            }

            // quick-xml reports `&amp;` and `&#38;` as their own events, which
            // is exactly what makes the two compare equal here: both resolve to
            // the same text, merged into the surrounding text node by `flatten`.
            XmlEvent::GeneralRef(reference) => {
                let resolved = match reference
                    .resolve_char_ref()
                    .map_err(|err| format!("bad character reference: {err}\n{xml}"))?
                {
                    Some(char) => char.to_string(),
                    None => {
                        let name = reference
                            .decode()
                            .map_err(|err| format!("bad entity reference: {err}\n{xml}"))?;
                        resolve_predefined_entity(&name)
                            .ok_or_else(|| format!("unknown entity &{name};\n{xml}"))?
                            .to_string()
                    }
                };

                if let Some(node) = stack.last_mut() {
                    node.children.push(Child::Text(resolved));
                }
            }

            XmlEvent::Eof => break,

            // Declaration, comments, processing instructions, doctype.
            _ => {}
        }
    }

    if !stack.is_empty() {
        return Err(format!("unclosed tags\n{xml}"));
    }

    Ok(roots)
}

fn flatten(node: &Node, out: &mut Vec<Event>) {
    out.push(Event::Start {
        name: node.name.clone(),
        attrs: node.attrs.clone(),
    });

    let has_element_children = node
        .children
        .iter()
        .any(|child| matches!(child, Child::Elem(_)));

    let mut index = 0;
    while index < node.children.len() {
        match &node.children[index] {
            Child::Elem(child) => {
                flatten(child, out);
                index += 1;
            }
            Child::Text(_) => {
                // Adjacent text and CDATA sections are one logical text node.
                let mut text = String::new();
                while let Some(Child::Text(chunk)) = node.children.get(index) {
                    text.push_str(chunk);
                    index += 1;
                }

                if !(has_element_children && text.trim().is_empty()) {
                    out.push(Event::Text(text));
                }
            }
        }
    }

    out.push(Event::End {
        name: node.name.clone(),
    });
}

fn resolved(result: ResolveResult) -> Option<String> {
    match result {
        ResolveResult::Unbound => None,
        ResolveResult::Bound(ns) => Some(to_string(ns.0)),
        ResolveResult::Unknown(prefix) => Some(format!(
            "<unbound prefix {}>",
            String::from_utf8_lossy(&prefix)
        )),
    }
}

fn is_namespace_declaration(key: &[u8]) -> bool {
    key == b"xmlns" || key.starts_with(b"xmlns:")
}

fn to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}
