pub trait Cacheable<'cache, F>
where
    Self: Sized,
{
    type Cache;

    fn new(from: F) -> anyhow::Result<Self::Cache>;
    fn restore(cache: &'cache Self::Cache) -> Self;
}
