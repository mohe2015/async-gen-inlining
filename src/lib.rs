// Licensed under MIT or Apache-2.0, from https://github.com/rust-lang/rust/blob/master/tests/ui/coroutine/async_gen_fn_iter.rs

// https://github.com/rust-lang/rust/issues/106765

// edition: 2024
// compile-flags: -Zunstable-options
// run-pass

#![feature(gen_blocks, async_iterator)]

use std::{future::Future, pin::Pin, task::{Context, Poll}};

// make sure that a ridiculously simple async gen fn works as an iterator.
struct Inline;

impl Future for Inline {
    type Output = i32;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(42i32)
    }
}

async gen fn foo() -> i32 {
    yield Inline.await;
    yield Inline.await;
    yield Inline.await;
    yield Inline.await;
    yield Inline.await;
    yield Inline.await;
    yield Inline.await;
    yield Inline.await;
    yield Inline.await;
}

async gen fn foo2() -> i32 {
    yield 42i32;
    yield 42i32;
    yield 42i32;
    yield 42i32;
    yield 42i32;
    yield 42i32;
    yield 42i32;
    yield 42i32;
    yield 42i32;
}
