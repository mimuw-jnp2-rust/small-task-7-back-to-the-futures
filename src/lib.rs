#![forbid(unsafe_code)]

pub mod executor;
pub mod timer;

use pin_project::pin_project;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

#[pin_project]
pub struct Map<Fut, F> {
    #[pin]
    fut: Fut,
    f: F,
}

impl<Fut, F> Map<Fut, F> {
    fn new(fut: Fut, f: F) -> Self {
        Map { fut, f }
    }
}

impl<Fut, F, T> Future for Map<Fut, F>
where
    Fut: Future,
    F: FnMut(Fut::Output) -> T,
{
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().project();
        let res = match this.fut.poll(cx) {
            Poll::Ready(v) => (this.f)(v),
            Poll::Pending => return Poll::Pending,
        };
        Poll::Ready(res)
    }
}

// You can change it to enum if you prefer
pub struct FlatMap<Fut1, F, Output>(PhantomData<(Fut1, F, Output)>);

impl<Fut1, F, Output> Future for FlatMap<Fut1, F, Output> {
    type Output = Output;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

// You can change it to enum if you prefer
pub struct Join<Fut1, Fut2, Output1, Output2>(PhantomData<(Fut1, Fut2, Output1, Output2)>);

impl<Fut1, Fut2, Output1, Output2> Future for Join<Fut1, Fut2, Output1, Output2> {
    type Output = (Output1, Output2);

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

pub trait FutureExt: Future {
    fn map<F, T>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Output) -> T,
        Self: Sized,
    {
        Map::new(self, f)
    }

    fn flat_map<Fut2, F>(self, _f: F) -> FlatMap<Self, F, Fut2::Output>
    where
        F: FnMut(Self::Output) -> Fut2,
        Fut2: Future,
        Self: Sized,
    {
        todo!()
    }

    fn join<Fut2>(self, _fut2: Fut2) -> Join<Self, Fut2, Self::Output, Fut2::Output>
    where
        Self: Sized,
        Fut2: Future,
    {
        todo!()
    }
}

impl<Fut> FutureExt for Fut where Fut: Future {}
