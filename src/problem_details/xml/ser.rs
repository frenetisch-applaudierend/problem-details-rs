//! Serialization of problem details to the XML shape RFC 9457 Appendix B
//! describes.
//!
//! quick-xml's serializer gets everything but sequences right. It writes a
//! sequence as repeated sibling elements — `<accounts>a</accounts>
//! <accounts>b</accounts>` — while Appendix B wants a container element holding
//! one `<i>` per item:
//!
//! ```xml
//! <accounts>
//!     <i>/account/12345</i>
//!     <i>/account/67890</i>
//! </accounts>
//! ```
//!
//! The repeated form is not merely different, it is ambiguous: Appendix B reads
//! an element with children as an object *"except for elements containing only
//! one or more child elements named `i`, which are considered arrays"*, so the
//! sibling form is an object with a repeated member name instead of an array.
//!
//! A sequence can sit anywhere inside a caller's extensions, so no attribute on
//! the fields of [`ProblemDetails`](crate::ProblemDetails) can fix this — the
//! wrapping has to be decided while serializing. Serialization therefore runs in
//! two steps: [`ValueSerializer`] turns any [`Serialize`] into the [`Value`]
//! tree below, which is where "this member is a sequence" becomes known, and
//! [`Value::write_document`] renders that tree.
//!
//! The other thing that tree decides is what to do with a value that is absent.
//! XML has no null, and Appendix B does not give one, so a `None` member is
//! omitted rather than written as an empty element — see [`Value::Null`].

use std::fmt::Display;

use serde::ser::{self, Serialize};

/// The element name Appendix B gives every item of an array.
const ITEM_NAME: &str = "i";

/// The prolog every body starts with.
const DECLARATION: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

/// Serialize `value` as an XML document rooted in an element named `root` that
/// declares `namespace` as its default namespace.
pub(super) fn to_document_string<T>(value: &T, root: &str, namespace: &str) -> Result<String, Error>
where
    T: ?Sized + Serialize,
{
    Ok(value
        .serialize(ValueSerializer)?
        .write_document(root, namespace))
}

/// A serialized value, in terms of the three shapes Appendix B distinguishes.
enum Value {
    /// A scalar, written as the text content of its element.
    Text(String),

    /// The absence of a value.
    ///
    /// XML has no null, and Appendix B does not invent one, so a `Null` member
    /// of an object is left out of the document entirely — see
    /// [`written_members`]. Writing it as an empty element would not be a
    /// neutral choice: `<absent/>` is a positive claim that the value is the
    /// empty string, and reads back as `Some("")` rather than `None`. Omission
    /// is how RFC 9457 expresses "no value" everywhere else; each of its own
    /// members is optional and left out when unset.
    ///
    /// An item of an array is the exception, since there is no element to omit
    /// and dropping it would change the length of the array. It keeps its
    /// `<i/>`, which is the least-bad of two imperfect options.
    Null,

    /// An array, written as one `<i>` element per item.
    Seq(Vec<Value>),

    /// An object, written as one child element per member.
    Map(Vec<(String, Value)>),
}

impl Value {
    /// Render this value as a complete document, prolog and all.
    fn write_document(&self, root: &str, namespace: &str) -> String {
        let mut out = String::from(DECLARATION);

        out.push('<');
        out.push_str(root);
        out.push_str(" xmlns=\"");
        out.push_str(&quick_xml::escape::escape(namespace));
        out.push('"');

        if self.is_empty() {
            out.push_str("/>");
            return out;
        }

        out.push('>');
        self.write_content(&mut out);
        write_end_tag(root, &mut out);

        out
    }

    /// Write this value as an element named `name`.
    fn write_element(&self, name: &str, out: &mut String) {
        out.push('<');
        out.push_str(name);

        if self.is_empty() {
            out.push_str("/>");
            return;
        }

        out.push('>');
        self.write_content(out);
        write_end_tag(name, out);
    }

    /// Write whatever this value contributes between its element's tags.
    fn write_content(&self, out: &mut String) {
        match self {
            Value::Text(text) => out.push_str(&quick_xml::escape::escape(text)),
            Value::Null => {}
            Value::Seq(items) => {
                for item in items {
                    item.write_element(ITEM_NAME, out);
                }
            }
            Value::Map(members) => {
                for (name, value) in written_members(members) {
                    value.write_element(name, out);
                }
            }
        }
    }

    /// Whether this value writes nothing between its element's tags, in which
    /// case the element is written in the equivalent `<name/>` form.
    fn is_empty(&self) -> bool {
        match self {
            Value::Text(text) => text.is_empty(),
            Value::Null => true,
            Value::Seq(items) => items.is_empty(),
            // Not `members.is_empty()`: an object whose members are all absent
            // writes no children either.
            Value::Map(members) => written_members(members).next().is_none(),
        }
    }
}

/// The members of an object that make it into the document, which is all of
/// them except the absent ones. See [`Value::Null`] for why those are dropped.
fn written_members(members: &[(String, Value)]) -> impl Iterator<Item = (&str, &Value)> {
    members
        .iter()
        .filter(|(_, value)| !matches!(value, Value::Null))
        .map(|(name, value)| (name.as_str(), value))
}

fn write_end_tag(name: &str, out: &mut String) {
    out.push_str("</");
    out.push_str(name);
    out.push('>');
}

/// Builds a [`Value`] out of anything serializable.
struct ValueSerializer;

impl ser::Serializer for ValueSerializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqBuilder;
    type SerializeTuple = SeqBuilder;
    type SerializeTupleStruct = SeqBuilder;
    type SerializeTupleVariant = TupleVariantBuilder;
    type SerializeMap = MapBuilder;
    type SerializeStruct = MapBuilder;
    type SerializeStructVariant = StructVariantBuilder;

    fn serialize_bool(self, value: bool) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_i8(self, value: i8) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_i16(self, value: i16) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_i32(self, value: i32) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_i64(self, value: i64) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_i128(self, value: i128) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_u8(self, value: u8) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_u16(self, value: u16) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_u32(self, value: u32) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_u64(self, value: u64) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_u128(self, value: u128) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_f32(self, value: f32) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_f64(self, value: f64) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_char(self, value: char) -> Result<Value, Error> {
        Ok(Value::Text(value.to_string()))
    }

    fn serialize_str(self, value: &str) -> Result<Value, Error> {
        Ok(Value::Text(value.to_owned()))
    }

    /// Written as the array of byte values a `Vec<u8>` would produce, rather
    /// than an encoding this serializer would have to invent.
    fn serialize_bytes(self, value: &[u8]) -> Result<Value, Error> {
        let items = value
            .iter()
            .map(|byte| Value::Text(byte.to_string()))
            .collect();

        Ok(Value::Seq(items))
    }

    fn serialize_none(self) -> Result<Value, Error> {
        Ok(Value::Null)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Value, Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Value, Error> {
        Ok(Value::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value, Error> {
        Ok(Value::Null)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Value, Error> {
        Ok(Value::Text(variant.to_owned()))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value, Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(Value::Map(vec![(
            variant.to_owned(),
            value.serialize(self)?,
        )]))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<SeqBuilder, Error> {
        Ok(SeqBuilder {
            items: Vec::with_capacity(len.unwrap_or_default()),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<SeqBuilder, Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<SeqBuilder, Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<TupleVariantBuilder, Error> {
        Ok(TupleVariantBuilder {
            variant,
            items: SeqBuilder {
                items: Vec::with_capacity(len),
            },
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<MapBuilder, Error> {
        Ok(MapBuilder {
            members: Vec::with_capacity(len.unwrap_or_default()),
            key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<MapBuilder, Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<StructVariantBuilder, Error> {
        Ok(StructVariantBuilder {
            variant,
            members: MapBuilder {
                members: Vec::with_capacity(len),
                key: None,
            },
        })
    }
}

/// Collects the items of a sequence, tuple or tuple struct.
struct SeqBuilder {
    items: Vec<Value>,
}

impl SeqBuilder {
    fn push<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.items.push(value.serialize(ValueSerializer)?);
        Ok(())
    }
}

impl ser::SerializeSeq for SeqBuilder {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::Seq(self.items))
    }
}

impl ser::SerializeTuple for SeqBuilder {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::Seq(self.items))
    }
}

impl ser::SerializeTupleStruct for SeqBuilder {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::Seq(self.items))
    }
}

/// A tuple variant, written as a single member named after the variant whose
/// value is the array of fields.
struct TupleVariantBuilder {
    variant: &'static str,
    items: SeqBuilder,
}

impl ser::SerializeTupleVariant for TupleVariantBuilder {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.items.push(value)
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(vec![(
            self.variant.to_owned(),
            Value::Seq(self.items.items),
        )]))
    }
}

/// Collects the members of a map or struct.
///
/// A member name becomes an element name, so it has to be a scalar; serde hands
/// map keys over as arbitrary values.
struct MapBuilder {
    members: Vec<(String, Value)>,

    /// The key of the entry currently being built, between the
    /// `serialize_key` and `serialize_value` calls that make up one entry.
    key: Option<String>,
}

impl MapBuilder {
    fn set_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        match key.serialize(ValueSerializer)? {
            Value::Text(name) => {
                self.key = Some(name);
                Ok(())
            }
            _ => Err(Error::new(
                "a member name must serialize to a scalar, since it becomes an element name",
            )),
        }
    }

    fn push_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let key = self
            .key
            .take()
            .ok_or_else(|| Error::new("a member value was serialized before its name"))?;

        self.members.push((key, value.serialize(ValueSerializer)?));
        Ok(())
    }
}

impl ser::SerializeMap for MapBuilder {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.set_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.push_value(value)
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(self.members))
    }
}

impl ser::SerializeStruct for MapBuilder {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, name: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.key = Some(name.to_owned());
        self.push_value(value)
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(self.members))
    }
}

/// A struct variant, written as a single member named after the variant whose
/// value is the object of fields.
struct StructVariantBuilder {
    variant: &'static str,
    members: MapBuilder,
}

impl ser::SerializeStructVariant for StructVariantBuilder {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, name: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.members.key = Some(name.to_owned());
        self.members.push_value(value)
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(vec![(
            self.variant.to_owned(),
            Value::Map(self.members.members),
        )]))
    }
}

/// An error raised while serializing a problem details to XML.
///
/// Almost always a [`serde::ser::Error::custom`] message from the extensions'
/// own `Serialize` impl; the serializer itself only rejects a member name that
/// cannot become an element name.
#[derive(Clone, Debug)]
pub(super) struct Error(String);

impl Error {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        Self::new(message.to_string())
    }
}
