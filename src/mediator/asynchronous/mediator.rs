use async_trait::async_trait;

use super::request::AsyncRequestHandler;

#[async_trait]
pub trait AsyncMediatorInternal<Ev> {
    async fn publish(&self, event: Ev);
    async fn send<Req>(&self, req: Req)
    where
        Self: AsyncRequestHandler<Req, Ev>,
        Req: Send;
}
