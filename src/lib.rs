#![doc(html_root_url = "https://docs.rs/mediator-sys/1.0.0")]
mod mediator;

pub mod prelude {
    #[cfg(feature = "async")]
    pub use super::mediator::asynchronous::{
        basic::{basic::BasicAsyncMediator, interface::*},
        contextaware::{contextaware::CxAwareAsyncMediator, interface::*},
    };
    pub use super::mediator::builder::*;
    pub use super::mediator::synchronous::basic::{basic::BasicMediator, interface::*};
}

#[cfg(test)]
mod test;
