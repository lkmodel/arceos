use std::env::args;
use std::io;
use std::path::PathBuf;
use std::time::Duration;
use futures::StreamExt;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> io::Result<()> {
    ::env_logger::init();

    let socket_addr = args().nth(1).expect("argument: QMP socket path");
    while !PathBuf::from(socket_addr.clone()).exists() {
        println!("Waiting for Qemu start");
        sleep(Duration::from_secs(5)).await;
    }
    #[cfg(unix)]
    let stream = qapi::futures::QmpStreamTokio::open_uds(socket_addr).await?;
    #[cfg(not(unix))]
    let stream = qapi::futures::QmpStreamTokio::open_tcp(socket_addr).await?;
    println!("{:#?}", stream.capabilities);
    let stream = stream.negotiate().await?;
    let (_, mut events) = stream.into_parts();

    while let Some(event) = events.next().await {
        println!("Got event {:#?}", event?);
    }

    Ok(())
}