use super::*;

pub struct Cache;

impl<C> GetContactByUserIdRepository<GetContactByUserIdKey, GetContactByUserIdValue, C> for Cache
where
    C: AsyncConnection,
{
    async fn get(
        // conn_repo: &(impl ConnectionRepository<C> + Send + Sync),
        conn_repo: &impl ConnectionRepository<C>,
        id: &GetContactByUserIdKey,
    ) -> Result<GetContactByUserIdValue, Error> {
        let mut conn = conn_repo.get_conn();
        todo!();
    }
}
