use super::*;

pub trait ConnectionRepository<C>: Send + Sync
where
    C: AsyncConnection,
{
    fn get_conn(&self) -> C;
}

#[derive(DeriveFrom)]
pub struct CacheRepository(Connection);

impl ConnectionRepository<MultiplexedConnection> for CacheRepository {
    fn get_conn(&self) -> MultiplexedConnection {
        self.0.to_owned()
    }
}

#[cfg(test)]
#[derive(DeriveFrom)]
pub struct MockCacheRepository(MockRedisConnection);

#[cfg(test)]
impl ConnectionRepository<MockRedisConnection> for MockCacheRepository {
    fn get_conn(&self) -> MockRedisConnection {
        self.0.to_owned()
    }
}

trait_set! {
    pub trait AsyncConnection = ConnectionLike + Send + Sync;
}
