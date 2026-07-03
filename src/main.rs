use std::sync::mpsc::{channel};
use app::server::Server;
pub mod app;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (term_tx, term_rx) = channel();
    ctrlc::set_handler(move || term_tx.send(()).expect("Error setting Ctrl-C handler"))?;

    let server = Server::new();
    server.start().await?;
    term_rx.recv()?;
    server.stop().await?;

    Ok(())
    
}