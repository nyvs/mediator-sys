use std::{fmt::Debug, sync::mpsc::TryRecvError};

use crate::mediator::listener::Listener;

/// Publish an event `Ev` from within a handler.
pub trait SyncMediatorInternal<Ev: Debug> {
    fn publish(&self, event: Ev);
}

/// Send a request `Req` for processing to the mediator.
/// This will call the handler.
pub trait SyncMediatorInternalHandle<Ev: Debug> {
    fn send<Req>(&self, req: Req)
    where
        Self: RequestHandler<Req, Ev>;
}

/// Process the next event `Ev` from the channel.
/// This will call all listeners with a clone of that event.
pub trait SyncMediatorInternalNext {
    fn next(&self) -> Result<(), TryRecvError>;
}

/// Handles the request `Req`.
/// Implemented by the user.
pub trait RequestHandler<Req, Res> {
    fn handle(&self, req: Req);
}

/// Basic builder fuctionality:
/// Adding a [`Listener`] to the builder.
pub trait BasicMediatorBuilderInterface<M, Ev> {
    fn add_listener<F>(self, f: F) -> Self
    where
        F: Listener<Ev>,
        Ev: Debug;
}
