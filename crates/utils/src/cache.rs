use serde::{Serialize, Deserialize};

pub trait Cacheable<'cache, F>
where
    Self: Sized,
{
    type Cache: Serialize + Deserialize<'cache>;

    fn new(from: F) -> anyhow::Result<Self::Cache>;
    fn restore(cache: &'cache Self::Cache) -> Self;
}
