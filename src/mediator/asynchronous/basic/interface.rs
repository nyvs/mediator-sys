use async_trait::async_trait;
use std::{fmt::Debug, sync::mpsc::TryRecvError};

/// Publish an event `Ev` asynchronously from within a handler.
#[async_trait]
pub trait AsyncMediatorInternal<Ev: Debug> {
    async fn publish(&self, event: Ev);
}

/// Send a request `Req` asynchronously for processing to the mediator.
/// This will call the handler.
#[async_trait]
pub trait AsyncMediatorInternalHandle<Ev: Debug> {
    async fn send<Req>(&self, req: Req)
    where
        Req: Send,
        Self: AsyncRequestHandler<Req, Ev>;
}

/// Process the next event `Ev` from the channel asynchronously.
/// This will call all listeners with a clone of that event.
#[async_trait]
pub trait AsyncMediatorInternalNext {
    async fn next(&self) -> Result<(), TryRecvError>;
}

/// Handles the request `Req` asynchronously.
/// Implemented by the user.
#[async_trait]
pub trait AsyncRequestHandler<Req, Res>
where
    Self: Sync,
{
    async fn handle(&self, req: Req);
}
