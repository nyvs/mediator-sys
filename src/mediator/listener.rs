use core::fmt::Debug;
pub trait Listener<Ev: Debug>: Fn(Ev) -> () + Send + 'static {}

impl<Ev> Debug for dyn Listener<Ev>
where
    Ev: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Listener Closure")
    }
}

impl<Ev, F> Listener<Ev> for F
where
    F: Fn(Ev) -> () + Send + 'static,
    Ev: Debug + Clone,
{
}
