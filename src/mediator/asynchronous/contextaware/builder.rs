use async_std::sync::Mutex;

use crate::{
    mediator::{builder::BuilderFlow, listener::Listener},
    prelude::{
        BasicAsyncMediator, BasicMediator, BasicMediatorBuilderInterface, BuilderInternal,
        CxAwareAsyncMediator, CxAwareMediatorBuilderInterface,
    },
};
use std::{fmt::Debug, sync::mpsc::channel};

pub struct CxAwareAsyncBuilder<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    mediator: BasicMediator<Ev>,
    dep: Option<Dep>,
}

impl<Dep, Ev> BuilderInternal<CxAwareAsyncMediator<Dep, Ev>, CxAwareAsyncBuilder<Dep, Ev>>
    for CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    fn builder() -> CxAwareAsyncBuilder<Dep, Ev> {
        CxAwareAsyncBuilder::<Dep, Ev> {
            mediator: BasicMediator::<Ev> {
                channel: channel(),
                listener: vec![],
            },
            dep: None,
        }
    }
}

impl<M, Dep, Ev> BasicMediatorBuilderInterface<M, Ev> for CxAwareAsyncBuilder<Dep, Ev>
where
    Dep: Debug,
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

impl<M, Dep, Ev> CxAwareMediatorBuilderInterface<M, Dep, Ev> for CxAwareAsyncBuilder<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    fn add_dependency(mut self, dep: Dep) -> Self
    where
        Ev: Debug,
    {
        self.dep = Some(dep);
        self
    }
}

impl<Dep, Ev> CxAwareAsyncBuilder<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    pub fn add_listener<F>(self, f: F) -> Self
    where
        F: Listener<Ev>,
    {
        <Self as BasicMediatorBuilderInterface<CxAwareAsyncMediator<Dep, Ev>, Ev>>::add_listener(
            self, f,
        )
    }

    pub fn add_dependency(self, dep: Dep) -> Self {
        <Self as CxAwareMediatorBuilderInterface<CxAwareAsyncMediator<Dep, Ev>, Dep, Ev>>::add_dependency(self, dep)
    }
}

#[derive(Debug)]
pub struct NoCxAvailable;

impl<Dep, Ev> BuilderFlow<CxAwareAsyncMediator<Dep, Ev>> for CxAwareAsyncBuilder<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    type Error = NoCxAvailable;
    fn build(self) -> Result<CxAwareAsyncMediator<Dep, Ev>, Self::Error> {
        Ok(CxAwareAsyncMediator {
            basic: BasicAsyncMediator {
                basic: Mutex::new(self.mediator),
            },
            dep: Mutex::new(self.dep.ok_or(NoCxAvailable)?),
        })
    }
}
