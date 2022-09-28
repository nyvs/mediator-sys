use async_trait::async_trait;

#[async_trait]
pub trait AsyncRequestHandler<Req, Res>
where
    Self: Sync,
{
    async fn handle(&self, req: Req);
}
