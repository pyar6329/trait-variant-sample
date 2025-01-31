use super::*;

#[trait_variant::make(Send)]
pub trait GetContactByUserIdRepository<K, V, C>
where
    K: CacheKey<u8>,
    V: CacheValue<u8>,
    C: AsyncConnection,
{
    async fn get(
        // conn_repo: &(impl ConnectionRepository<C> + Send + Sync),
        conn_repo: &impl ConnectionRepository<C>,
        id: &K,
    ) -> Result<V, Error>;
}

#[derive(Clone, Debug, DeriveDeref, DeriveFrom, Eq, PartialEq, Ord, PartialOrd)]
pub struct GetContactByUserIdKey(u8);

#[derive(Clone, Debug, DeriveDeref, DeriveFrom)]
pub struct GetContactByUserIdValue(u8);

impl CacheKey<u8> for GetContactByUserIdKey {
    const PREFIX_KEY_NAME: CacheKeyPrefix = CacheKeyPrefix::ContactByUserId;
}

impl CacheValue<u8> for GetContactByUserIdValue {
    fn ttl(&self) -> u8 {
        12
    }
}

impl TryFrom<i64> for GetContactByUserIdValue {
    type Error = Error;
    fn try_from(_: i64) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl FromRedisValue for GetContactByUserIdValue {
    fn from_redis_value(_: &RedisValue) -> RedisResult<Self> {
        todo!()
    }
}
