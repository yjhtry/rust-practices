use anyhow::Result;
use kv::{ProstServerStream, Service, SledTable};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let db = sled::open("temp/sled")?;

    let service: Service =
        Service::new(SledTable::new(db)).fn_before_send(|res| match res.message.as_ref() {
            "" => info!("Get message is empty!",),
            message => info!("Get message: {}", message),
        });
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
