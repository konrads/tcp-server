# Simple TCP server

[![build](https://github.com/konrads/tcp-server/workflows/build/badge.svg)](https://github.com/konrads/tcp-server/actions/workflows/build.yml)
[![build](workflows/build/badge.svg)](actions/workflows/build.yml)

- takes `RequestHandler` impls for result calculations
- unmarshalls `Request`s and marshalls `Response`s, carrying id across.

## Usage

```bash
# in terminal 1
cargo run server

# in terminal 2
cargo run clinet
```
