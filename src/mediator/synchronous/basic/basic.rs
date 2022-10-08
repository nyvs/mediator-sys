use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::mediator::listener::Listener;
use core::fmt::Debug;

use super::interface::{
    RequestHandler, SyncMediatorInternal, SyncMediatorInternalHandle, SyncMediatorInternalNext,
};

#[derive(Debug)]
pub struct BasicMediator<Ev>
where
    Ev: Debug,
{
    pub(crate) channel: (Sender<Ev>, Receiver<Ev>),
    pub(crate) listener: Vec<Box<dyn Listener<Ev>>>,
}

impl<Ev> SyncMediatorInternal<Ev> for BasicMediator<Ev>
where
    Ev: Debug,
{
    fn publish(&self, event: Ev) {
        self.channel.0.send(event).ok();
    }
}

impl<Ev> SyncMediatorInternalHandle<Ev> for BasicMediator<Ev>
where
    Ev: Debug,
{
    fn send<Req>(&self, req: Req)
    where
        Self: RequestHandler<Req, Ev>,
    {
        <Self as RequestHandler<Req, Ev>>::handle(self, req);
    }
}

impl<Ev> SyncMediatorInternalNext for BasicMediator<Ev>
where
    Ev: Debug + Clone,
{
    fn next(&self) -> Result<(), TryRecvError> {
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
