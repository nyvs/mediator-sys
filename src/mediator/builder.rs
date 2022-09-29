use super::listener::Listener;
use std::fmt::Debug;

pub trait BuilderInternal
where
    Self: Sized,
{
    fn builder() -> Builder<Self>;
}

pub trait BasicBuilderInterface<M, Ev> {
    fn add_listener<F>(self, f: F) -> Self
    where
        F: Listener<Ev>,
        Ev: Clone + Debug;
}

pub struct Builder<M> {
    pub(crate) mediator: M,
}

impl<M> Builder<M> {
    pub fn build(self) -> M {
        self.mediator
    }
}
