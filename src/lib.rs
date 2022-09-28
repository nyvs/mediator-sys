mod mediator;

pub mod prelude {
    pub use super::mediator::builder::{
        Builder,
        BuilderInternal,
        BasicBuilderInterface,
    };
    pub use super::mediator::synchronous::{
        basic::BasicMediator, 
        request::RequestHandler,
        mediator::SyncMediatorInternal,
    };
    #[cfg(feature = "async")]
    pub use super::mediator::asynchronous::{
        basic::BasicAsyncMediator, 
        request::AsyncRequestHandler,
        mediator::AsyncMediatorInternal,
    };
}

#[cfg(test)]
mod test;
