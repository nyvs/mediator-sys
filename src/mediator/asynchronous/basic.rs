use std::sync::mpsc::TryRecvError;

use async_std::sync::Mutex;
use async_trait::async_trait;

use crate::mediator::synchronous::{basic::BasicMediator, mediator::SyncMediatorInternal};

use super::{mediator::AsyncMediatorInternal, request::AsyncRequestHandler};

pub struct BasicAsyncMediator<Ev>
where
    Ev: Clone,
{
    default: Mutex<BasicMediator<Ev>>,
}

#[async_trait]
impl<Ev> AsyncMediatorInternal<Ev> for BasicAsyncMediator<Ev>
where
    Ev: Clone + Send,
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
    Ev: Clone + Send,
{
    fn from(value: BasicMediator<Ev>) -> Self {
        Self {
            default: Mutex::new(value),
        }
    }
}

impl<Ev> BasicAsyncMediator<Ev>
where
    Ev: Clone,
{
    pub async fn next(&self) -> Result<(), TryRecvError> {
        let m = self.default.lock().await;
        m.next()
    }
}
