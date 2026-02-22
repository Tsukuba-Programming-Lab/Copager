use serde::{Serialize, Deserialize};

use crate::result::Result as CoResult;

pub trait Cacheable<F>
where
    Self: Sized,
{
    type Cache: Serialize + for<'de> Deserialize<'de>;

    fn cache(from: F) -> CoResult<Self::Cache>;
    fn restore(cache: Self::Cache) -> Self;
}
