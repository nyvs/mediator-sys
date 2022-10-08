use std::sync::mpsc::TryRecvError;

use async_std::sync::Mutex;
use async_trait::async_trait;
use std::fmt::Debug;

use crate::mediator::synchronous::basic::{
    basic::BasicMediator,
    interface::{SyncMediatorInternal, SyncMediatorInternalNext},
};

use super::interface::{
    AsyncMediatorInternal, AsyncMediatorInternalHandle, AsyncMediatorInternalNext,
    AsyncRequestHandler,
};

#[derive(Debug)]
pub struct BasicAsyncMediator<Ev>
where
    Ev: Debug,
{
    pub(crate) basic: Mutex<BasicMediator<Ev>>,
}

#[async_trait]
impl<Ev> AsyncMediatorInternal<Ev> for BasicAsyncMediator<Ev>
where
    Ev: Debug + Send,
{
    async fn publish(&self, event: Ev) {
        let m = self.basic.lock().await;
        m.publish(event)
    }
}

#[async_trait]
impl<Ev> AsyncMediatorInternalHandle<Ev> for BasicAsyncMediator<Ev>
where
    Ev: Debug,
{
    async fn send<Req>(&self, req: Req)
    where
        Self: AsyncRequestHandler<Req, Ev>,
        Req: Send,
    {
        <Self as AsyncRequestHandler<Req, Ev>>::handle(self, req).await
    }
}

#[async_trait]
impl<Ev> AsyncMediatorInternalNext for BasicAsyncMediator<Ev>
where
    Ev: Debug + Clone + Send,
{
    async fn next(&self) -> Result<(), TryRecvError> {
        let m = self.basic.lock().await;
        m.next()
    }
}
