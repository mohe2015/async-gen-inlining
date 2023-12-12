// Licensed under MIT or Apache-2.0, from https://github.com/rust-lang/rust/blob/master/tests/ui/coroutine/async_gen_fn_iter.rs

// edition: 2024
// compile-flags: -Zunstable-options
// run-pass

#![feature(gen_blocks, async_iterator)]

// make sure that a ridiculously simple async gen fn works as an iterator.

async fn pause() {
    // this doesn't actually do anything, lol
}

async fn one() -> i32 {
    1
}

async fn two() -> i32 {
    2
}

async gen fn foo() -> i32 {
    yield one().await;
    pause().await;
    yield two().await;
    pause().await;
    yield 3;
    pause().await;
}

async fn async_main() {
    let mut async_iterator = std::pin::pin!(foo());
   
    while let Some(value) = async_iterator.as_mut().next().await {
        println!("{value}");
    }
}

// ------------------------------------------------------------------------- //
// Implementation Details Below...

use std::pin::Pin;
use std::task::*;
use std::async_iter::AsyncIterator;
use std::future::Future;

trait AsyncIterExt {
    fn next(&mut self) -> Next<'_, Self>;
}

impl<T> AsyncIterExt for T {
    fn next(&mut self) -> Next<'_, Self> {
        Next { s: self }
    }
}

struct Next<'s, S: ?Sized> {
    s: &'s mut S,
}

impl<'s, S: AsyncIterator> Future for Next<'s, S> where S: Unpin {
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        Pin::new(&mut *self.s).poll_next(cx)
    }
}

pub fn noop_waker() -> Waker {
    let raw = RawWaker::new(std::ptr::null(), &NOOP_WAKER_VTABLE);

    // SAFETY: the contracts for RawWaker and RawWakerVTable are upheld
    unsafe { Waker::from_raw(raw) }
}

const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

unsafe fn noop_clone(_p: *const ()) -> RawWaker {
    RawWaker::new(std::ptr::null(), &NOOP_WAKER_VTABLE)
}

unsafe fn noop(_p: *const ()) {}

fn main() {
    let mut fut = std::pin::pin!(async_main());

    // Poll loop, just to test the future...
    let waker = noop_waker();
    let ctx = &mut Context::from_waker(&waker);

    loop {
        match fut.as_mut().poll(ctx) {
            Poll::Pending => {}
            Poll::Ready(()) => break,
        }
    }
}