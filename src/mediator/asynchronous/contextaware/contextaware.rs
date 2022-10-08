use std::sync::mpsc::TryRecvError;

use async_std::sync::Mutex;
use async_trait::async_trait;
use std::fmt::Debug;

use crate::mediator::asynchronous::basic::{
    basic::BasicAsyncMediator,
    interface::{AsyncMediatorInternal, AsyncMediatorInternalNext},
};

use super::interface::{CxAwareAsyncMediatorInternalHandle, CxAwareAsyncRequestHandler};

#[derive(Debug)]
pub struct CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    pub(crate) basic: BasicAsyncMediator<Ev>,
    pub(crate) dep: Mutex<Dep>,
}

#[async_trait]
impl<Dep, Ev> AsyncMediatorInternal<Ev> for CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug + Send,
    Ev: Debug + Send,
{
    async fn publish(&self, event: Ev) {
        self.basic.publish(event).await
    }
}

#[async_trait]
impl<Dep, Ev> CxAwareAsyncMediatorInternalHandle<Dep, Ev> for CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug + Send + Sync,
    Ev: Debug + Send,
{
    async fn send<Req>(&self, req: Req)
    where
        Self: CxAwareAsyncRequestHandler<Dep, Req, Ev>,
        Req: Send,
    {
        let m = self.dep.lock().await;
        <Self as CxAwareAsyncRequestHandler<Dep, Req, Ev>>::handle(self, req, &m).await
    }
}

#[async_trait]
impl<Dep, Ev> AsyncMediatorInternalNext for CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug + Send,
    Ev: Debug + Clone + Send,
{
    async fn next(&self) -> Result<(), TryRecvError> {
        self.basic.next().await
    }
}
