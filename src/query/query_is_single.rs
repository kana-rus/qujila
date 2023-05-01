use std::{
    future::Future,
    pin::pin,
    task::Poll, marker::PhantomData,
};
use crate::{
    condition::Condition,
    Error,
    pool, Table,
};


pub struct is_single<T: Table> {
    __table__: PhantomData<fn()->T>,
    condition: Condition,
}
impl<T: Table> Future for is_single<T> {
    type Output = Result<bool, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "SELECT COUNT(id) FROM (SELECT 1 as id FROM {} {} LIMIT 1)",
            T::TABLE_NAME,
            self.condition,
        );
        let fetch_future = pin!(
            sqlx::query_as::<_, (i64,)>(&sql)
                .fetch_one(pool())
        );

        match fetch_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok((count,))) => Poll::Ready(Ok(count == 1)),
        }
    }
}
