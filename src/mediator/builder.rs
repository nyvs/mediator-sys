/// Trait for creating a builder
/// that implements [`BuilderFlow`]
/// for a mediator `M`.
pub trait BuilderInternal<M, Flow>
where
    Flow: BuilderFlow<M>,
    Self: Sized,
{
    fn builder() -> Flow;
}

/// A [`BuilderFlow`] is generic over `M`
/// which is the mediator that will be
/// built by [`BuilderFlow::build()`].
pub trait BuilderFlow<M>
where
    Self: Sized,
{
    fn build(self) -> M;
}

/// Trait for creating a builder
/// that implements [`TryBuilderFlow`]
/// for a mediator `M`.
pub trait TryBuilderInternal<M, Flow>
where
    Flow: TryBuilderFlow<M>,
    Self: Sized,
{
    fn builder() -> Flow;
}

/// A [`TryBuilderFlow`] is generic over `M`
/// which is the mediator that will be tried to be
/// built by [`TryBuilderFlow::build()`].
pub trait TryBuilderFlow<M>
where
    Self: Sized,
{
    type Error;
    fn build(self) -> Result<M, Self::Error>;
}
