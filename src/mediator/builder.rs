pub trait BuilderInternal<M, Flow>
where
    Flow: BuilderFlow<M>,
    Self: Sized,
{
    fn builder() -> Flow;
}

pub trait BuilderFlow<M>
where
    Self: Sized,
{
    fn build(self) -> M;
}

pub trait TryBuilderInternal<M, Flow>
where
    Flow: TryBuilderFlow<M>,
    Self: Sized,
{
    fn builder() -> Flow;
}

pub trait TryBuilderFlow<M>
where
    Self: Sized,
{
    type Error;
    fn build(self) -> Result<M, Self::Error>;
}
