use serde::{Serialize, Deserialize};

pub trait Cacheable<F>
where
    Self: Sized,
{
    type Cache: Serialize + for<'de> Deserialize<'de>;

    fn cache(from: F) -> anyhow::Result<Self::Cache>;
    fn restore(cache: Self::Cache) -> Self;
}
