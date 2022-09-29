use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

use super::{mediator::SyncMediatorInternal, request::RequestHandler};
use crate::mediator::{
    builder::{BasicBuilderInterface, Builder, BuilderInternal},
    listener::Listener,
};
use core::fmt::Debug;

#[derive(Debug)]
pub struct BasicMediator<Ev>
where
    Ev: Clone + Debug,
{
    pub(crate) channel: (Sender<Ev>, Receiver<Ev>),
    pub(crate) listener: Vec<Box<dyn Listener<Ev>>>,
}

impl<Ev> SyncMediatorInternal<Ev> for BasicMediator<Ev>
where
    Ev: Clone + Debug,
{
    fn publish(&self, event: Ev) {
        self.channel.0.send(event).ok();
    }

    fn send<Req>(&self, req: Req)
    where
        Self: RequestHandler<Req, Ev>,
    {
        <Self as RequestHandler<Req, Ev>>::handle(self, req)
    }
}

impl<Ev> BuilderInternal for BasicMediator<Ev>
where
    Ev: Clone + Debug,
{
    fn builder() -> Builder<Self> {
        Builder {
            mediator: BasicMediator::<Ev> {
                channel: channel(),
                listener: vec![],
            },
        }
    }
}

impl<M, Ev> BasicBuilderInterface<M, Ev> for Builder<BasicMediator<Ev>>
where
    Ev: Clone + Debug,
{
    fn add_listener<F>(mut self, f: F) -> Self
    where
        F: Listener<Ev> + Send,
    {
        self.mediator.listener.push(Box::new(f));
        self
    }
}

impl<Ev> Builder<BasicMediator<Ev>>
where
    Ev: Clone + Debug,
{
    pub fn add_listener<F>(self, f: F) -> Self
    where
        F: Listener<Ev> + Send,
    {
        <Self as BasicBuilderInterface<BasicMediator<Ev>, Ev>>::add_listener(self, f)
    }
}

impl<Ev, F> Listener<Ev> for F
where
    F: Fn(Ev) -> () + Send + 'static,
    Ev: Clone + Debug,
{
}

impl<Ev> BasicMediator<Ev>
where
    Ev: Clone + Debug,
{
    pub fn next(&self) -> Result<(), TryRecvError> {
        match self.channel.1.try_recv() {
            Ok(ev) => {
                for listener in self.listener.iter() {
                    listener(ev.clone())
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
