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

### Runtime Features

Choose the appropriate runtime feature for your project:

- `runtime-tokio` (default): Use with Tokio runtime
- `runtime-smol`: Use with Smol runtime

## Configuration

### Using API

Configure matchers file serving using the builder pattern:

```rust,ignore

```

### vetis.yaml

```yaml
type: matchers_path
uri: /matchers
extensions: \.(html|css|js|png|jpg)$
directory: ./public
index_files: [index.html]
cache:
    max_file_size: 10485760
    ttl: 60s
    tti: 10s
    capacity: 1000
```

### Configuration Options

#### StaticHookConfig

- **uri**: The URI prefix for serving matchers files (required)
- **extensions**: Comma-separated list of file extensions to serve (required)
- **directory**: The filesystem directory containing matchers files (required)
- **index_files**: Optional list of index file names to serve for directory requests
- **cache**: Optional cache configuration

## Runtime-Specific Modules

Based on your selected runtime feature, import the appropriate module:

```rust
// For Tokio runtime
#[cfg(feature = "runtime-tokio")]
use vetis_matchers::tokio;

// For Smol runtime
#[cfg(feature = "runtime-smol")]
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
