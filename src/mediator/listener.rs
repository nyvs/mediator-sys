use core::fmt::Debug;
pub trait Listener<Ev: Clone + Debug>: Fn(Ev) -> () + Send + 'static {}

impl<Ev> Debug for dyn Listener<Ev>
where
    Ev: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Listener Closure")
    }
}
