# problem_details

![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

RFC 9457 / RFC 7807 problem details for HTTP APIs.

This crate provides the [`ProblemDetails`](https://docs.rs/problem_details/latest/problem_details/struct.ProblemDetails.html)
struct which implements the [RFC 9457](https://www.rfc-editor.org/rfc/rfc9457.html) / [RFC 7807](https://www.rfc-editor.org/rfc/rfc7807.html)
problem details specification.

It supports serializing and deserializing problem details using JSON, and provides integration
with the [axum (0.8)](https://crates.io/crates/axum) and [poem (3.1)](https://crates.io/crates/poem) web frameworks.

## Usage

The following example shows how to create a problem details object that produces
the [example JSON from the RFC](https://www.rfc-editor.org/rfc/rfc9457.html#name-the-problem-details-json-ob).

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

## Extensions

[Extensions](https://www.rfc-editor.org/rfc/rfc9457.html#name-extension-members) can be added
to the problem details object using the [`with_extensions`](https://docs.rs/problem_details/latest/problem_details/struct.ProblemDetails.html#method.with_extensions)
method. The extensions are passed using a struct defining the extension fields.

During serialization, the extension fields are flattened into the problem details object.

```rust
use problem_details::ProblemDetails;

#[derive(serde::Serialize, serde::Deserialize)]
struct MyExt {
    foo: String,
    bar: u32,
}

let details = ProblemDetails::new()
    .with_title("Extensions test")
    .with_extensions(MyExt {
        foo: "Hello".to_string(),
        bar: 42,
    });

let json = serde_json::to_value(&details).unwrap();

assert_eq!(json, serde_json::json!({
  "title": "Extensions test",
  "foo": "Hello",
  "bar": 42
}));
```

To deserialize with extensions, provide the extensions type as the generic
parameter to the [`ProblemDetails`](https://docs.rs/problem_details/latest/problem_details/struct.ProblemDetails.html) struct.

```rust
let details: ProblemDetails<MyExt> = serde_json::from_str(json).unwrap();
```

If you need dynamic extensions, you can use a `HashMap` as extensions object.

## Features

- **serde**: Enables serde support for the `ProblemDetails` struct (_enabled by default_)
- **json**:  Enables serialization to JSON when using web framework integrations
             (_enabled by default, implies `serde`)
- **xml**:   Enables serialization to XML when using web framework integrations
             (_implies `serde`_)
- **axum**:  Enables integration with the [`axum`](https://crates.io/crates/axum)
             web framework, enabling to return `ProblemDetails` as responses.
- **poem**:  Enables integration with the [`poem`](https://crates.io/crates/poem)
             web framework, enabling to return `ProblemDetails` as responses and errors.

## Caveats

This crate is not fully compliant with RFC 9457, because it fails to deserialize
JSON values containing properties with incorrect types (required by
[Chapter 3.1 of the RFC](https://www.rfc-editor.org/rfc/rfc9457.html#name-members-of-a-problem-detail)).

## License

Licensed under either of

- [Apache License, Version 2.0](https://opensource.org/license/apache-2-0/)
- [The MIT License](https://opensource.org/license/mit/)

at your option.
