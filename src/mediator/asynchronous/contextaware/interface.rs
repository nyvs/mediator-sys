use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait CxAwareAsyncMediatorInternalHandle<Dep, Ev: Debug> {
    async fn send<Req>(&self, req: Req)
    where
        Req: Send,
        Self: CxAwareAsyncRequestHandler<Dep, Req, Ev>;
}

#[async_trait]
pub trait CxAwareAsyncRequestHandler<Dep, Req, Res> {
    async fn handle(&self, req: Req, dep: &Dep);
}

pub trait CxAwareMediatorBuilderInterface<M, Dep, Ev> {
    fn add_dependency(self, dep: Dep) -> Self
    where
        Ev: Debug;
}
