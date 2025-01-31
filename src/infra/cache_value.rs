use super::*;

pub trait CacheValue<T>
where
    Self: Sized + From<T> + Deref<Target = T> + FromRedisValue + TryFrom<i64, Error = Error>,
    T: Clone + DeserializeOwned + Serialize,
{
    fn ttl(&self) -> u8;
}
