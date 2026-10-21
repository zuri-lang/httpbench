# httpbench spec

An automated HTTP benchmarking suite comparing Zuri's standard HTTP server (`libs/http`) against top programming languages and JIT runtimes across HTTP/1.1 and HTTP/2 cleartext (`h2c`).

## Protocols

- `http1`: HTTP/1.1 persistent connections (`h2load --h1`)
- `http2`: Cleartext HTTP/2 with prior knowledge connection preface (`h2load -p h2c`)

## Workloads

1. **`plaintext`**: `GET /plaintext`
   - Response: `200 OK`, `Content-Type: text/plain`, Body: `Hello, World!` (13 bytes)
   - Evaluates minimal framing, parsing, and dispatch overhead.
2. **`json`**: `GET /json`
   - Response: `200 OK`, `Content-Type: application/json`, Body: `{"message":"Hello, World!"}` (27 bytes)
   - Evaluates JSON serialization and content-type header generation.
3. **`echo`**: `POST /echo`
   - Request: 100 bytes payload (`application/octet-stream`)
   - Response: `200 OK`, `Content-Type: application/octet-stream`, Body: request payload echoed back
   - Evaluates request body streaming, buffering, and response writing.

## Language and JIT Runtime Setup

| Library | Language / Runtime | Implementation | Notes |
| :--- | :--- | :--- | :--- |
| `zuri` | Zuri (Default / Bayelsa) | `libs/http` (`http.server`) | Default (Bayelsa direct, single-threaded), `compression = false` |
| `zuri-kebbi` | Zuri (Kebbi Baseline) | `libs/http` (`http.server`) | `ZURI_JIT_KEBBI=1` (single-threaded), `compression = false` |
| `zuri-serve` | Zuri (Multi-Worker) | `libs/http` (`http.serve`) | Multi-worker isolate pool (`cpu_count`), `compression = false` |
| `node-raw` | Node.js v24 (V8) | `node:http` & `node:http2` | Standard library HTTP/1.1 and HTTP/2 |
| `py-raw` | CPython 3.14 | `hypercorn` (ASGI) | Pure-async HTTP/1.1 & HTTP/2 server |
| `pypy-raw` | PyPy 3.11 (JIT) | `hypercorn` (ASGI) | PyPy JIT async HTTP/1.1 & HTTP/2 server |
| `rb-raw` | Ruby 4.0.6 (YJIT) | `falcon` (Async) | Fiber-based async HTTP/1.1 & HTTP/2 server |

## Important Configuration Notes

- **Compression**: In Zuri, HTTP responses are compressed with Brotli by default. For a fair measurement against raw baseline servers, `server.compression = false` is explicitly set, eliminating Brotli CPU encoding overhead.
- **HTTP/2**: Zuri natively speaks HTTP/2 cleartext on the same port when receiving the connection preface, matching Node, Go, Rust, Python, and Ruby.

