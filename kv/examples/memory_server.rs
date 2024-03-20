use anyhow::Result;
use kv::{MemTable, ProstServerStream, Service};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let service: Service = Service::new(MemTable::new());
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;

    info!("Start listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;

        info!("Client {:?} connected", addr);

        let svc = service.clone();

        tokio::spawn(async move {
            let stream = ProstServerStream::new(stream, svc);
            tokio::spawn(async move { stream.process().await });

            info!("Client {:?} disconnected", addr);
        });
    }
}
