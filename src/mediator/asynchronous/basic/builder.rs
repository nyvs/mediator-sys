use async_std::sync::Mutex;

use crate::mediator::{
    asynchronous::basic::basic::BasicAsyncMediator,
    builder::{BuilderFlow, BuilderInternal},
    listener::Listener,
    synchronous::basic::{basic::BasicMediator, interface::BasicMediatorBuilderInterface},
};
use std::{fmt::Debug, sync::mpsc::channel};

pub struct BasicAsyncBuilder<Ev>
where
    Ev: Debug,
{
    mediator: BasicMediator<Ev>,
}

impl<Ev> BuilderInternal<BasicAsyncMediator<Ev>, BasicAsyncBuilder<Ev>> for BasicAsyncMediator<Ev>
where
    Ev: Debug,
{
    fn builder() -> BasicAsyncBuilder<Ev> {
        BasicAsyncBuilder::<Ev> {
            mediator: BasicMediator::<Ev> {
                channel: channel(),
                listener: vec![],
            },
        }
    }
}

impl<M, Ev> BasicMediatorBuilderInterface<M, Ev> for BasicAsyncBuilder<Ev>
where
    Ev: Debug,
{
    fn add_listener<F>(mut self, f: F) -> Self
    where
        F: Listener<Ev>,
    {
        self.mediator.listener.push(Box::new(f));
        self
    }
}

impl<Ev> BasicAsyncBuilder<Ev>
where
    Ev: Debug,
{
    pub fn add_listener<F>(self, f: F) -> Self
    where
        F: Listener<Ev>,
    {
        <Self as BasicMediatorBuilderInterface<BasicMediator<Ev>, Ev>>::add_listener(self, f)
    }
}

impl<Ev> BuilderFlow<BasicAsyncMediator<Ev>> for BasicAsyncBuilder<Ev>
where
    Ev: Debug,
{
    fn build(self) -> BasicAsyncMediator<Ev> {
        BasicAsyncMediator {
            basic: Mutex::new(self.mediator),
        }
    }
}
