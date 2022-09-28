mod mediator;

pub mod prelude {
    #[cfg(feature = "async")]
    pub use super::mediator::asynchronous::{
        basic::BasicAsyncMediator, mediator::AsyncMediatorInternal, request::AsyncRequestHandler,
    };
    pub use super::mediator::builder::{BasicBuilderInterface, Builder, BuilderInternal};
    pub use super::mediator::synchronous::{
        basic::BasicMediator, mediator::SyncMediatorInternal, request::RequestHandler,
    };
}

#[cfg(test)]
mod test;
