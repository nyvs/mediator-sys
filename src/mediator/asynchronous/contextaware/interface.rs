use async_trait::async_trait;
use std::fmt::Debug;

/// Send a request `Req` asynchronously for processing to the mediator.
/// This will call the handler.
/// The handler here is context-dependent.
#[async_trait]
pub trait CxAwareAsyncMediatorInternalHandle<Dep, Ev: Debug> {
    async fn send<Req>(&self, req: Req)
    where
        Req: Send,
        Self: CxAwareAsyncRequestHandler<Dep, Req, Ev>;
}

/// Handles the request `Req` asynchronously.
/// Implemented by the user.
/// Gives access to the dependency `Dep`.
#[async_trait]
pub trait CxAwareAsyncRequestHandler<Dep, Req, Res> {
    async fn handle(&self, req: Req, dep: &Dep);
}

/// Advanced builder fuctionality:
/// Adding a dependency `dep` to the builder.
pub trait CxAwareMediatorBuilderInterface<M, Dep, Ev> {
    fn add_dependency(self, dep: Dep) -> Self
    where
        Ev: Debug;
}
