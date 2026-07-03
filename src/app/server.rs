use std::sync::Arc;
use libp2p::futures::future::select;
use libp2p::futures::StreamExt;
use thiserror::Error;
use tokio::{select, sync::Mutex, task::{JoinError, JoinSet}};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Error)]
pub enum Error {
    #[error("task join error: {0}")]
    TaskJoin(#[from] JoinError),
}

pub type ServerResult<T> = Result<T, Error>;
pub struct Server {
    cancel_token: CancellationToken,
    subtasks: Arc<Mutex<JoinSet<()>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {cancel_token: CancellationToken::new(), subtasks: Arc::new(Mutex::new(JoinSet::new()))}
    }

    pub async fn start(&self) -> ServerResult<()> {
        let mut join_set = self.subtasks.lock().await;
        let cancel_token = self.cancel_token.clone();
        join_set.spawn(async move {
            loop {
                select! {
                    _ = cancel_token.cancelled() => {
                        println!("task cancelled");
                        break;
                    }
                }
            }
        });
        Ok(())
    }

    pub async fn stop(&self) -> ServerResult<()> {
        println!("stop server");
        self.cancel_token.cancel();
        let mut tasks = self.subtasks.lock().await;
        while let Some(result) = tasks.join_next().await {
            result?;
        }
        Ok(())

    }
}