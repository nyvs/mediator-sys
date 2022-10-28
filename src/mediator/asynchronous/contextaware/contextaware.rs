use std::sync::mpsc::TryRecvError;

use async_std::sync::Mutex;
use async_trait::async_trait;
use std::fmt::Debug;

use crate::asynchronous::basic::BasicAsyncMediator;

use super::*;

/// Context aware async mediator for asynchronous environments with events of type `Ev`.
///
/// Uses an underlying [`BasicAsyncMediator`] for base functionality
/// and a [`async_std::sync::Mutex`] to store the user-defined dependency `Dep`.
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
///     .add_listener(move |ev| {
///         /* Your listening logic */
///     })
///     .add_listener(move |ev| {
///         /* Your listening logic */
///     })
///     .add_dependency(MyContext::default())
///     .build();
///
#[cfg(feature = "async")]
#[derive(Debug)]
pub struct CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug,
    Ev: Debug,
{
    pub(crate) basic: BasicAsyncMediator<Ev>,
    pub(crate) dep: Mutex<Dep>,
}

#[async_trait]
impl<Dep, Ev> AsyncMediatorInternal<Ev> for CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug + Send,
    Ev: Debug + Send,
{
    /// Publishes an event `Ev` asynchronously.
    ///
    /// This method instructs the underlying [`BasicAsyncMediator`]
    /// to publish a user-defined event.
    ///
    /// It should be used within [`CxAwareAsyncRequestHandler::handle()`].
    ///
    /// You need to await the `Future` using `.await`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mediator_sys::asynchronous::contextaware::*;
    /// use async_trait::async_trait;
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
    /// struct Request(u32);
    ///
    /// #[async_trait]
    /// impl CxAwareAsyncRequestHandler<MyContext, Request, MyEvent> for CxAwareAsyncMediator<MyContext, MyEvent> {
    ///     async fn handle(&self, req: Request, dep: &MyContext) {
    ///         let my_context: u32 = *dep.0;
    ///         match req.0 {
    ///             1 => self.publish(MyEvent::One).await,
    ///             2 => self.publish(MyEvent::Two).await,
    ///             _ => ()
    ///         };
    ///     }
    /// }
    ///
    async fn publish(&self, event: Ev) {
        self.basic.publish(event).await
    }
}

#[async_trait]
impl<Dep, Ev> CxAwareAsyncMediatorInternalHandle<Dep, Ev> for CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug + Send + Sync,
    Ev: Debug + Send,
{
    /// Send a request of type `Req` to the mediator asynchronously.
    ///
    /// The request will be processed internally by [`CxAwareAsyncRequestHandler::handle()`].
    /// This is why it is required to implement [`CxAwareAsyncRequestHandler`] for [`CxAwareAsyncMediator`].
    ///
    /// You need to await the `Future` using `.await`.
    ///
    async fn send<Req>(&self, req: Req)
    where
        Self: CxAwareAsyncRequestHandler<Dep, Req, Ev>,
        Req: Send,
    {
        let m = self.dep.lock().await;
        <Self as CxAwareAsyncRequestHandler<Dep, Req, Ev>>::handle(self, req, &m).await
    }
}

#[async_trait]
impl<Dep, Ev> AsyncMediatorInternalNext for CxAwareAsyncMediator<Dep, Ev>
where
    Dep: Debug + Send,
    Ev: Debug + Clone + Send,
{
    /// Process the next published event `Ev` asynchronously.
    ///
    /// This method instructs the underlying [`BasicAsyncMediator`]
    /// to process the next event.
    ///
    /// See [`BasicAsyncMediator::next()`] for more info.
    ///
    /// You need to await the `Future` using `.await`.
    ///
    async fn next(&self) -> Result<(), TryRecvError> {
        self.basic.next().await
    }
}
