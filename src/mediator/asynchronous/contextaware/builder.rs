use async_std::sync::Mutex;

use crate::mediator::{
    asynchronous::{
        basic::basic::BasicAsyncMediator,
        contextaware::{
            contextaware::CxAwareAsyncMediator, interface::CxAwareMediatorBuilderInterface,
        },
    },
    builder::{TryBuilderFlow, TryBuilderInternal},
    listener::Listener,
    synchronous::basic::{basic::BasicMediator, interface::BasicMediatorBuilderInterface},
};
use std::{fmt::Debug, sync::mpsc::channel};

/// The [`CxAwareAsyncBuilder`] helps you to create a [`CxAwareAsyncMediator`].
///
/// The [`CxAwareAsyncBuilder`] is part of the builder pattern.
/// It has three functionalities. The first one is adding a [`Listener`] via
/// [`CxAwareAsyncBuilder::add_listener()`].
/// Secondly, a dependency `Dep` can be added via [`CxAwareAsyncBuilder::add_dependency()`].
/// This must be done in order to receive a [`CxAwareAsyncMediator`] from [`TryBuilderFlow::build()`].
/// The third functionality is the mandatory [`TryBuilderFlow::build()`], which returns
/// a [`Result`] of type [`Result<CxAwareAsyncMediator<Dep, Ev>, Self::Error>`].
///
pub struct CxAwareAsyncBuilder<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    mediator: BasicMediator<Ev>,
    dep: Option<Dep>,
}

impl<Dep, Ev> TryBuilderInternal<CxAwareAsyncMediator<Dep, Ev>, CxAwareAsyncBuilder<Dep, Ev>>
    for CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    /// Creates a [`CxAwareAsyncBuilder`] with the goal of producing a [`CxAwareAsyncMediator`].
    ///
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
    /// Adds a user-defined listener to the [`CxAwareAsyncBuilder`].
    ///
    /// To be able to supply a closure that implements [`Listener`],
    /// it must satisfy [`Send`] and `'static` bounds.
    ///
    /// Also it must be a `Fn(Ev)` with a return type of `()`
    /// where `Ev` is the user-defined event type
    /// that must be [`Clone`] and [`Debug`].
    ///
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
    /// Adds a user-defined dependency of type `Dep` to the [`CxAwareAsyncBuilder`].
    ///
    /// The dependency will act as a context and become available in [`super::CxAwareAsyncRequestHandler::handle()`].
    ///
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
    /// Adds a user-defined listener to the [`CxAwareAsyncBuilder`].
    ///
    /// The supplied type must be a [`Listener`].
    /// As such, it must implement [`Send`] and `Fn(Ev)`,
    /// besides being `'static`.
    ///
    /// As a side note, here, `Ev` is the user-defined event type
    /// that must be [`Clone`] and [`Debug`].
    ///
    /// Note: The following example will add a [`Listener`] to the builder,
    /// but the result of `.build()` here will be an `Err` value.
    /// This is because in order to receive a valid [`CxAwareAsyncMediator`]
    /// you need to add a dependency. See [`CxAwareAsyncBuilder::add_dependency()`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mediator_sys::asynchronous::contextaware::*;
    /// use std::sync::Arc;
    ///
    /// #[derive(Debug, Clone)]
    /// enum MyEvent {
    ///     One,
    ///     Two
    /// }
    ///
    /// #[derive(Debug, Default)]
    /// struct MyContext(Arc<u32>);
    ///
    /// let mediator = CxAwareAsyncMediator::<MyContext, MyEvent>::builder()
    ///     .add_listener(|ev| {
    ///         /* Your listening logic */
    ///     })
    ///     .build();
    ///
    pub fn add_listener<F>(self, f: F) -> Self
    where
        F: Listener<Ev>,
    {
        <Self as BasicMediatorBuilderInterface<CxAwareAsyncMediator<Dep, Ev>, Ev>>::add_listener(
            self, f,
        )
    }

    /// Adds a user-defined dependency of type `Dep` to the [`CxAwareAsyncBuilder`].
    ///
    /// The dependency will act as a context and become available in [`super::CxAwareAsyncRequestHandler::handle()`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mediator_sys::asynchronous::contextaware::*;
    /// use std::sync::Arc;
    ///
    /// #[derive(Debug, Clone)]
    /// enum MyEvent {
    ///     One,
    ///     Two
    /// }
    ///
    /// #[derive(Debug, Default)]
    /// struct MyContext(Arc<u32>);
    ///
    /// let mediator = CxAwareAsyncMediator::<MyContext, MyEvent>::builder()
    ///     .add_dependency(MyContext::default())
    ///     .build();
    ///
    pub fn add_dependency(self, dep: Dep) -> Self {
        <Self as CxAwareMediatorBuilderInterface<CxAwareAsyncMediator<Dep, Ev>, Dep, Ev>>::add_dependency(self, dep)
    }
}

#[derive(Debug)]
pub struct NoCxAvailable;

impl<Dep, Ev> TryBuilderFlow<CxAwareAsyncMediator<Dep, Ev>> for CxAwareAsyncBuilder<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    type Error = NoCxAvailable;
    /// Builds the [`CxAwareAsyncMediator`] and returns it.
    ///
    /// Because [`CxAwareAsyncMediator`] implements [`TryBuilderInternal`],
    /// which in turn means, that the [`CxAwareAsyncBuilder`] implements [`TryBuilderFlow`]
    /// this method will return a [`Result<CxAwareAsyncMediator<Dep, Ev>, Self::Error>`] as stated by the return type.
    /// Note that here `Self::Error` is of type [`NoCxAvailable`], which means that no dependecy was added in
    /// the process of building.
    ///
    fn build(self) -> Result<CxAwareAsyncMediator<Dep, Ev>, Self::Error> {
        Ok(CxAwareAsyncMediator {
            basic: BasicAsyncMediator {
                basic: Mutex::new(self.mediator),
            },
            dep: Mutex::new(self.dep.ok_or(NoCxAvailable)?),
        })
    }
}
