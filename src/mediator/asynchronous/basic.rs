use std::sync::mpsc::TryRecvError;

use crate::mediator::synchronous::{basic::BasicMediator, mediator::SyncMediatorInternal};
use async_std::sync::Mutex;
use async_trait::async_trait;
use std::fmt::Debug;

use super::{mediator::AsyncMediatorInternal, request::AsyncRequestHandler};

#[derive(Debug)]
pub struct BasicAsyncMediator<Ev>
where
    Ev: Clone + Debug,
{
    default: Mutex<BasicMediator<Ev>>,
}

#[async_trait]
impl<Ev> AsyncMediatorInternal<Ev> for BasicAsyncMediator<Ev>
where
    Ev: Clone + Send + Debug,
{
    async fn publish(&self, event: Ev) {
        let m = self.default.lock().await;
        m.publish(event)
    }

    async fn send<Req>(&self, req: Req)
    where
        Self: AsyncRequestHandler<Req, Ev>,
        Req: Send,
    {
        <Self as AsyncRequestHandler<Req, Ev>>::handle(self, req).await
    }
}

impl<Ev> From<BasicMediator<Ev>> for BasicAsyncMediator<Ev>
where
    Ev: Clone + Send + Debug,
{
    fn from(value: BasicMediator<Ev>) -> Self {
        Self {
            default: Mutex::new(value),
        }
    }
}

impl<Ev> BasicAsyncMediator<Ev>
where
    Ev: Clone + Debug,
{
    pub async fn next(&self) -> Result<(), TryRecvError> {
        let m = self.default.lock().await;
        m.next()
    }
}
