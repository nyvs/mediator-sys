#![doc(html_root_url = "https://docs.rs/mediator-sys/0.1.0")]
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
