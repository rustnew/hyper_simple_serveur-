use std::any;
use std::convert::Infallible;
use std::net::SocketAddr;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

struct  Person {
    nom: String,
    prenon : String,
}

async  fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("hello world"))))
}

#[tokio::main]
async  fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync >> {

    let port  = SocketAddr::from(([127, 0, 0, 1], 3000));
    let  listener  = TcpListener::bind(port).await?;

    loop {
        let (stream, _)= listener.accept().await?;

        let  io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let  Err(err) = http1::Builder::new()
            .serve_connection(io, service_fn(hello))
            .await{
                eprint!("error serving connection {:#}", err);
            }

        });
    }
}
