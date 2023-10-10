![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

# problem_details

RFC 9457 / RFC 7807 problem details for HTTP APIs.

This crate can be used to represent a problem details
object as defined in RFC 9457 (which obsoletes RFC 7807).

The `ProblemDetails` struct includes the standard fields
(`type`, `status`, `title`, `detail`, `instance`),
as well as type-safe custom extensions.

## Extensions

To add extensions, you need to define a struct that holds the extension
fields, and use this struct as the generic parameter for `ProblemDetails<Ext>`.
Using `with_extensions`), the type is adjusted automatically for you.

Extension fields are flattened into the problem details object when serialized.

```rust
use problem_details::ProblemDetails;

struct MyExt {
    foo: String,
    bar: u32,
}

let details = ProblemDetails::new()
    .with_extensions(MyExt {
        foo: "Hello".to_string(),
        bar: 42,
    });

// details is of type ProblemDetails<MyExt>
let typecheck: ProblemDetails<MyExt> = details;
```

If you need dynamic extensions, you can use a `HashMap`
as extensions object.

```rust
use std::collections::HashMap;
use problem_details::ProblemDetails;

let mut extensions = HashMap::<String, serde_json::Value>::new();
extensions.insert("foo".to_string(), serde_json::json!("Hello"));
extensions.insert("bar".to_string(), serde_json::json!(42));

let details = ProblemDetails::new()
   .with_extensions(extensions);

// details is of type ProblemDetails<HashMap<String, serde_json::Value>>
let typecheck: ProblemDetails<HashMap<String, serde_json::Value>> = details;
```

## Example

The following example shows how to create a problem details object that produces
the [example JSON from the RFC](https://www.rfc-editor.org/rfc/rfc9457.pdf#name-the-problem-details-json-ob).

```rust
use http::Uri;
use problem_details::ProblemDetails;

#[derive(serde::Serialize)]
struct OutOfCreditExt {
   balance: u32,
   accounts: Vec<String>,
}

let details = ProblemDetails::new()
    .with_type(Uri::from_static("https://example.com/probs/out-of-credit"))
    .with_title("You do not have enough credit.")
    .with_detail("Your current balance is 30, but that costs 50.")
    .with_instance(Uri::from_static("/account/12345/msgs/abc"))
    .with_extensions(OutOfCreditExt {
        balance: 30,
        accounts: vec![
            "/account/12345".to_string(),
            "/account/67890".to_string(),
        ],
    });

let json = serde_json::to_value(&details).unwrap();

assert_eq!(json, serde_json::json!({
  "type": "https://example.com/probs/out-of-credit",
  "title": "You do not have enough credit.",
  "detail": "Your current balance is 30, but that costs 50.",
  "instance": "/account/12345/msgs/abc",
  "balance": 30,
  "accounts": [
    "/account/12345",
    "/account/67890"
  ]
}));
```

## Features

- **serde**: Enables serde support for the `ProblemDetails` struct (_enabled by default_)
- **axum**: Enables axum IntoResponse types for the `ProblemDetails` struct (_implies `serde`_)
- **xml**: Enables serde XML support for the `ProblemDetails` struct using
           [`quick-xml`](https://crates.io/crates/quick-xml) (_implies `serde`_)

## Caveats

This crate is not fully compliant with the RFC, because it fails to deserialize
JSON values containing properties with incorrect types (required by
[Chapter 3.1 of the RFC](https://www.rfc-editor.org/rfc/rfc9457.pdf#name-members-of-a-problem-detail)).

License: MIT OR Apache-2.0
