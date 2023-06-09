use std::{
    future::Future,
    task::Poll,
    pin::pin, marker::PhantomData,
};
use crate::{
    __feature__,
    Error,
    pool,
    Model, Table,
    condition::Condition,
};


pub struct First<T: Table, M: Model> {
    __table__: PhantomData<fn()->T>,
    __model__: PhantomData<fn()->M>,
    condition: Condition,
}
impl<T: Table, M: Model> Future for First<T, M> {
    type Output = Result<M, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "SELECT {} FROM {} {} LIMIT 1",
            M::SELECT_COLUMNS,
            T::TABLE_NAME,
            self.condition,
        );
        let query_future = pin!(sqlx::query::<__feature__::DB>(&sql).fetch_one(pool()));

        match query_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e))  => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(row)) => Poll::Ready(M::from_row(&row)),
        }
    }
}
impl<T: Table, M: Model> First<T, M> {
    #[inline] pub(crate) fn new(condition: Condition) -> Self {
        Self { __table__: PhantomData, __model__: PhantomData, condition }
    }
}
