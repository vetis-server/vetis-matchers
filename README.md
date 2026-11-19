# vetis-matchers

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-matchers)](https://crates.io/crates/vetis-matchers) [![crates.io](https://img.shields.io/crates/v/vetis-matchers?style=flat-square)](https://crates.io/crates/vetis-matchers) [![Build Status](https://github.com/vetis-server/vetis-matchers/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis-matchers/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-matchers) [![Documentation](https://docs.rs/vetis-matchers/badge.svg)](https://docs.rs/vetis-matchers/latest/vetis-matchers) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis-matchers/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis-matchers/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis-matchers)

Request and response matchers for Vetis hooks.

## Features

- header, query, path and other matchers
- expression matcher using CEL

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vetis-matchers = "0.1.1-beta.2"
```

## Matchers

### Client Ip

- **ranges**: A list of paths to match request uri against to (required)

### Expression

- **expression**: A list of paths to match request uri against to (required)

### Header

- **field**: Header field name (required)
- **value**: Header field value (required)

### Header Regex

- **field**: Header field name (required)

### Method

- **verbs**: A list of paths to match request uri against to (required)

### Path

Path matcher allow route request based on request uri path.

Below you have some configuration options:

- **paths**: A list of paths to match request uri against to (required)

### Path Regex

- **pattern**: A regex pattern to match request path against to (required)

### Query

- **param**: A list of paths to match request uri against to (required)
- **value**: Value pattern

### Scheme

- **protos**: A list of paths to match request uri against to (required)

### URL Pattern

- **scheme**: A list of paths to match request uri against to (required)

### Version

- **specs**: A list of paths to match request uri against to (required)

### Examples

#### Using API

Configure matchers file serving using the builder pattern:

```rust,ignore
use vetis_matchers::request::{PathMatcher, PathMatcherConfig};

PathMatcher::new(
  PathMatcherConfig::builder()
    .add_path("/api*")
    .add_path("/healthz")
    .build()?
);
```

#### vetis.yaml

```yaml
type: "path"
paths:
  - "/api*"
  - "/healthz"
```

### Path Regex

- **pattern**: A regex pattern to match with request uri (required)

### Query

- **paths**: A list of paths to match request uri against to (required)

### Scheme

- **paths**: A list of paths to match request uri against to (required)

### Url Pattern

- **paths**: A list of paths to match request uri against to (required)

### Version

- **paths**: A list of paths to match request uri against to (required)

## Runtime-Specific Modules

Based on your selected runtime feature, import the appropriate module:

```rust
use vetis_matchers::smol;
```

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
