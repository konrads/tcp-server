# Simple TCP server

- takes `RequestHandler` impls for result calculations
- unmarshalls `Request`s and marshalls `Response`s, carrying id across.

## Usage

```bash
# in terminal 1
cargo run server

# in terminal 2
cargo run clinet
```