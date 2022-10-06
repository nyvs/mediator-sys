use async_trait::async_trait;
use std::{fmt::Debug, sync::mpsc::TryRecvError};

#[async_trait]
pub trait AsyncMediatorInternal<Ev: Debug> {
    async fn publish(&self, event: Ev);
}

#[async_trait]
pub trait AsyncMediatorInternalHandle<Ev: Debug> {
    async fn send<Req>(&self, req: Req)
    where
        Req: Send,
        Self: AsyncRequestHandler<Req, Ev>;
}

#[async_trait]
pub trait AsyncMediatorInternalNext {
    async fn next(&self) -> Result<(), TryRecvError>;
}

#[async_trait]
pub trait AsyncRequestHandler<Req, Res>
where
    Self: Sync,
{
    async fn handle(&self, req: Req);
}
