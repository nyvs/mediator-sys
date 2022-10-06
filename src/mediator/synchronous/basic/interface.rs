use std::{fmt::Debug, sync::mpsc::TryRecvError};

use crate::mediator::listener::Listener;

pub trait SyncMediatorInternal<Ev: Debug> {
    fn publish(&self, event: Ev);
}

pub trait SyncMediatorInternalHandle<Ev: Debug> {
    fn send<Req>(&self, req: Req)
    where
        Self: RequestHandler<Req, Ev>;
}

pub trait SyncMediatorInternalNext {
    fn next(&self) -> Result<(), TryRecvError>;
}

pub trait RequestHandler<Req, Res> {
    fn handle(&self, req: Req);
}

pub trait BasicMediatorBuilderInterface<M, Ev> {
    fn add_listener<F>(self, f: F) -> Self
    where
        F: Listener<Ev>,
        Ev: Debug;
}
