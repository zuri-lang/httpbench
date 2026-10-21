import sys
import asyncio
from hypercorn.config import Config
from hypercorn.asyncio import serve

port = int(sys.argv[1]) if len(sys.argv) > 1 else 9000
proto = sys.argv[2] if len(sys.argv) > 2 else "both"

async def app(scope, receive, send):
    if scope["type"] == "lifespan":
        while True:
            message = await receive()
            if message["type"] == "lifespan.startup":
                await send({"type": "lifespan.startup.complete"})
            elif message["type"] == "lifespan.shutdown":
                await send({"type": "lifespan.shutdown.complete"})
                return

    path = scope.get("path", "/")
    method = scope.get("method", "GET")

    if path == "/health":
        body = b"ok"
        headers = [(b"content-type", b"text/plain"), (b"content-length", b"2")]
    elif path == "/plaintext":
        body = b"Hello, World!"
        headers = [(b"content-type", b"text/plain"), (b"content-length", b"13")]
    elif path == "/json":
        body = b'{"message":"Hello, World!"}'
        headers = [(b"content-type", b"application/json"), (b"content-length", str(len(body)).encode())]
    elif path == "/echo" and method == "POST":
        body = b""
        more_body = True
        while more_body:
            message = await receive()
            body += message.get("body", b"")
            more_body = message.get("more_body", False)
        headers = [(b"content-type", b"application/octet-stream"), (b"content-length", str(len(body)).encode())]
    else:
        body = b"Not Found"
        headers = [(b"content-type", b"text/plain"), (b"content-length", str(len(body)).encode())]
        await send({"type": "http.response.start", "status": 404, "headers": headers})
        await send({"type": "http.response.body", "body": body})
        return

    await send({"type": "http.response.start", "status": 200, "headers": headers})
    await send({"type": "http.response.body", "body": body})

config = Config()
config.bind = [f"127.0.0.1:{port}"]
config.insecure_bind = [f"127.0.0.1:{port}"]
config.accesslog = None
config.errorlog = None

asyncio.run(serve(app, config))

