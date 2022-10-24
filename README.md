# Simple TCP server

[![build](../../workflows/build/badge.svg)](../../actions/workflows/build.yml)

- takes `RequestHandler` impls for result calculations
- unmarshalls `Request`s and marshalls `Response`s, carrying id across.

## Usage

```bash
# in terminal 1
cargo run server

# in terminal 2
cargo run client
# or
echo '{"id":1,"payload":{"x":99,"y":1}}' | nc localhost 3333
```
