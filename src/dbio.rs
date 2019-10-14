use futures::future::{BoxFuture, FutureExt};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct DBIO<T>(BoxFuture<'static, crate::Result<T>>);

impl<T> DBIO<T>
{
    pub fn new<F>(inner: F) -> Self
    where
        F: Future<Output = crate::Result<T>> + Send + 'static,
    {
        Self(inner.boxed())
    }
}

impl<T> Future for DBIO<T>
{
    type Output = crate::Result<T>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(ctx)
    }
}
