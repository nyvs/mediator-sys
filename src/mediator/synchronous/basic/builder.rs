use crate::{
    mediator::{builder::BuilderFlow, listener::Listener},
    prelude::{BasicMediator, BasicMediatorBuilderInterface, BuilderInternal},
};
use std::{fmt::Debug, sync::mpsc::channel};
pub struct BasicBuilder<Ev>
where
    Ev: Debug,
{
    mediator: BasicMediator<Ev>,
}

impl<Ev> BuilderInternal<BasicMediator<Ev>, BasicBuilder<Ev>> for BasicMediator<Ev>
where
    Ev: Debug,
{
    fn builder() -> BasicBuilder<Ev> {
        BasicBuilder::<Ev> {
            mediator: BasicMediator::<Ev> {
                channel: channel(),
                listener: vec![],
            },
        }
    }
}

impl<M, Ev> BasicMediatorBuilderInterface<M, Ev> for BasicBuilder<Ev>
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

impl<Ev> BasicBuilder<Ev>
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

impl<Ev> BuilderFlow<BasicMediator<Ev>> for BasicBuilder<Ev>
where
    Ev: Debug,
{
    type Error = ();
    fn build(self) -> Result<BasicMediator<Ev>, Self::Error> {
        Ok(self.mediator)
    }
}
