use std::sync::mpsc::{Sender, Receiver, channel};

use crate::mediator::builder::{Builder, BuilderInternal, BasicBuilderInterface};
use super::{mediator::SyncMediatorInternal, request::RequestHandler};

pub struct BasicMediator<Ev>
where Ev: Clone {
    pub(crate) channel: (Sender<Ev>, Receiver<Ev>),
    pub(crate) listener: Vec<Box<dyn Fn(Ev) -> () + Send>>,
}

impl<Ev> SyncMediatorInternal<Ev> for BasicMediator<Ev>
where Ev: Clone {
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
where Ev: Clone {
    fn builder() -> Builder<Self> {
        Builder { mediator: BasicMediator::<Ev> {
            channel: channel(),
            listener: vec![],
        }}
    }
}

impl<M, Ev> BasicBuilderInterface<M, Ev> for Builder<BasicMediator<Ev>>
where Ev: Clone {
    fn add_listener<F>(mut self, f: F) -> Self where F: Send + Fn(Ev) -> () + 'static {
        self.mediator.listener.push(Box::new(f));
        self
    }
}

impl<Ev> Builder<BasicMediator<Ev>>
where Ev: Clone {
    pub fn add_listener<F>(self, f: F) -> Self where F: Send + Fn(Ev) -> () + 'static {
        <Self as BasicBuilderInterface<BasicMediator<Ev>, Ev>>::add_listener(self, f)
    }
}

impl<Ev> BasicMediator<Ev>
where Ev: Clone {
    pub fn next(&self) {
        if let Some(ev) = self.channel.1.try_recv().ok() {
            for listener in self.listener.iter() {
                listener(ev.clone())
            }
        }
    }
}
