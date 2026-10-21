use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;

async fn handle(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = req.uri().path();
    let method = req.method();

    if method == Method::GET && path == "/health" {
        let mut res = Response::new(Full::new(Bytes::from_static(b"ok")));
        res.headers_mut().insert("content-type", "text/plain".parse().unwrap());
        Ok(res)
    } else if method == Method::GET && path == "/plaintext" {
        let mut res = Response::new(Full::new(Bytes::from_static(b"Hello, World!")));
        res.headers_mut().insert("content-type", "text/plain".parse().unwrap());
        Ok(res)
    } else if method == Method::GET && path == "/json" {
        let mut res = Response::new(Full::new(Bytes::from_static(b"{\"message\":\"Hello, World!\"}")));
        res.headers_mut().insert("content-type", "application/json".parse().unwrap());
        Ok(res)
    } else if method == Method::POST && path == "/echo" {
        let body = req.into_body().collect().await.unwrap().to_bytes();
        let mut res = Response::new(Full::new(body));
        res.headers_mut().insert("content-type", "application/octet-stream".parse().unwrap());
        Ok(res)
    } else {
        let mut res = Response::new(Full::new(Bytes::from_static(b"Not Found")));
        *res.status_mut() = StatusCode::NOT_FOUND;
        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let port: u16 = if args.len() > 1 { args[1].parse().unwrap_or(9000) } else { 9000 };
    let proto = if args.len() > 2 { args[2].clone() } else { "http1".to_string() };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;
    println!("Rust HTTP server listening on {} ({})", addr, proto);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let proto_clone = proto.clone();

        tokio::task::spawn(async move {
            if proto_clone == "http2" {
                let builder = hyper::server::conn::http2::Builder::new(TokioExecutor::new());
                let _ = builder.serve_connection(io, service_fn(handle)).await;
            } else {
                let builder = hyper::server::conn::http1::Builder::new();
                let _ = builder.serve_connection(io, service_fn(handle)).await;
            }
        });
    }
}

