use anyhow::Result;
use kv::{MemTable, ProstServerStream, Service, TlsServerAcceptor};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";

    let server_cert = include_str!("../fixtures/server.cert");
    let server_key = include_str!("../fixtures/server.key");
    let acceptor = TlsServerAcceptor::new(server_cert, server_key, None)?;

    let service = Service::new(MemTable::new());
    let listener = TcpListener::bind(addr).await?;

    info!("Start listening on {}", addr);

    loop {
        let tls = acceptor.clone();
        let (stream, addr) = listener.accept().await?;

        info!("Client {:?} connected", addr);

        let stream = tls.accept(stream).await?;

        let stream = ProstServerStream::new(stream, service.clone());
        tokio::spawn(async move { stream.process().await });
    }
}
