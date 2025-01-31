use super::*;

pub trait CacheKey<T>
where
    Self: Sized + From<T> + Deref<Target = T>,
    T: Clone + Serialize,
{
    const PREFIX_KEY_NAME: CacheKeyPrefix;
}

#[derive(Copy, Clone, Debug, StrumDisplay, EnumIs)]
pub enum CacheKeyPrefix {
    ContactByUserId,
}
