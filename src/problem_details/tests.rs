use http::{StatusCode, Uri};
use serde_json::json;

use crate::{ProblemDetails, ProblemType};

#[test]
#[allow(clippy::unit_cmp)]
fn from_status() {
    let details = ProblemDetails::from_status_code(StatusCode::NOT_FOUND);

    assert_eq!(
        details.r#type.unwrap_or_default(),
        ProblemType::from(Uri::from_static("about:blank"))
    );
    assert_eq!(details.status, Some(StatusCode::NOT_FOUND));
    assert_eq!(details.title, Some("Not Found".to_string()));
    assert_eq!(details.detail, None);
    assert_eq!(details.instance, None);
    assert_eq!(details.extensions, ());
}

#[test]
fn fully_configured() {
    #[derive(Debug, PartialEq, Eq)]
    struct Extensions {
        foo: String,
        bar: u32,
    }

    let details = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_status(StatusCode::INTERNAL_SERVER_ERROR)
        .with_title("Test Title")
        .with_detail("Test Detail")
        .with_instance(Uri::from_static("test:instance"))
        .with_extensions(Extensions {
            foo: "Foo".to_string(),
            bar: 42,
        });

    assert_eq!(
        details.r#type,
        Some(ProblemType::from(Uri::from_static("test:type")))
    );
    assert_eq!(details.status, Some(StatusCode::INTERNAL_SERVER_ERROR));
    assert_eq!(details.title, Some("Test Title".to_string()));
    assert_eq!(details.detail, Some("Test Detail".to_string()));
    assert_eq!(details.instance, Some(Uri::from_static("test:instance")));
    assert_eq!(
        details.extensions,
        Extensions {
            foo: "Foo".to_string(),
            bar: 42
        }
    );
}

#[test]
fn to_string() {
    let empty = ProblemDetails::new();

    let type_only = ProblemDetails::new().with_type(Uri::from_static("test:type"));
    let status_only = ProblemDetails::new().with_status(StatusCode::NOT_FOUND);
    let title_only = ProblemDetails::new().with_title("Test Title");
    let detail_only = ProblemDetails::new().with_detail("Test Detail");

    let type_status = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_status(StatusCode::NOT_FOUND);
    let type_title = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_title("Test Title");
    let type_detail = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_detail("Test Detail");
    let status_title = ProblemDetails::new()
        .with_status(StatusCode::NOT_FOUND)
        .with_title("Test Title");
    let status_detail = ProblemDetails::new()
        .with_status(StatusCode::NOT_FOUND)
        .with_detail("Test Detail");
    let title_detail = ProblemDetails::new()
        .with_title("Test Title")
        .with_detail("Test Detail");

    let type_status_title = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_status(StatusCode::NOT_FOUND)
        .with_title("Test Title");
    let type_status_detail = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_status(StatusCode::NOT_FOUND)
        .with_detail("Test Detail");
    let type_title_detail = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_title("Test Title")
        .with_detail("Test Detail");
    let status_title_detail = ProblemDetails::new()
        .with_status(StatusCode::NOT_FOUND)
        .with_title("Test Title")
        .with_detail("Test Detail");

    let full = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_status(StatusCode::NOT_FOUND)
        .with_title("Test Title")
        .with_detail("Test Detail");

    assert_eq!("[about:blank]", empty.to_string());

    assert_eq!("[test:type]", type_only.to_string());
    assert_eq!("[about:blank 404] Not Found", status_only.to_string());
    assert_eq!("[about:blank] Test Title", title_only.to_string());
    assert_eq!("[about:blank] Test Detail", detail_only.to_string());

    assert_eq!("[test:type 404] Not Found", type_status.to_string());
    assert_eq!("[test:type] Test Title", type_title.to_string());
    assert_eq!("[test:type] Test Detail", type_detail.to_string());
    assert_eq!("[about:blank 404] Test Title", status_title.to_string());
    assert_eq!(
        "[about:blank 404] Not Found: Test Detail",
        status_detail.to_string()
    );
    assert_eq!(
        "[about:blank] Test Title: Test Detail",
        title_detail.to_string()
    );

    assert_eq!("[test:type 404] Test Title", type_status_title.to_string());
    assert_eq!(
        "[test:type 404] Not Found: Test Detail",
        type_status_detail.to_string()
    );
    assert_eq!(
        "[test:type] Test Title: Test Detail",
        type_title_detail.to_string()
    );
    assert_eq!(
        "[about:blank 404] Test Title: Test Detail",
        status_title_detail.to_string()
    );

    assert_eq!("[test:type 404] Test Title: Test Detail", full.to_string());
}

#[cfg(feature = "serde")]
#[test]
fn serialize_empty() {
    let empty = ProblemDetails::new();

    let serialized = serde_json::to_value(empty).unwrap();

    let expected = json!({});

    assert_eq!(expected, serialized);
}

#[cfg(feature = "serde")]
#[test]
fn serialize_filled() {
    #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    struct Extensions {
        foo: String,
        bar: u32,
    }

    let details = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_status(StatusCode::INTERNAL_SERVER_ERROR)
        .with_title("Test Title")
        .with_detail("Test Detail")
        .with_instance(Uri::from_static("test:instance"))
        .with_extensions(Extensions {
            foo: "Foo".to_string(),
            bar: 42,
        });

    let serialized = serde_json::to_value(details).unwrap();

    let expected = json!({
        "type": "test:type",
        "status": 500,
        "title": "Test Title",
        "detail": "Test Detail",
        "instance": "test:instance",
        "foo": "Foo",
        "bar": 42
    });

    assert_eq!(expected, serialized);
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_empty() {
    let empty = json!({});

    let deserialized = serde_json::from_value(empty).unwrap();

    let expected = ProblemDetails::new();

    assert_eq!(expected, deserialized);
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_filled() {
    #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    struct Extensions {
        foo: String,
        bar: u32,
    }

    let filled = json!({
        "type": "test:type",
        "status": 500,
        "title": "Test Title",
        "detail": "Test Detail",
        "instance": "test:instance",
        "foo": "Foo",
        "bar": 42
    });

    let deserialized: ProblemDetails<Extensions> = serde_json::from_value(filled).unwrap();

    let expected = ProblemDetails::new()
        .with_type(Uri::from_static("test:type"))
        .with_status(StatusCode::INTERNAL_SERVER_ERROR)
        .with_title("Test Title")
        .with_detail("Test Detail")
        .with_instance(Uri::from_static("test:instance"))
        .with_extensions(Extensions {
            foo: "Foo".to_string(),
            bar: 42,
        });

    assert_eq!(expected, deserialized);
}
