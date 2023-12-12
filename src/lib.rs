// Licensed under MIT or Apache-2.0, from https://github.com/rust-lang/rust/blob/master/tests/ui/coroutine/async_gen_fn_iter.rs

// https://github.com/rust-lang/rust/issues/106765

// edition: 2024
// compile-flags: -Zunstable-options
// run-pass
#![no_std]
#![feature(gen_blocks, async_iterator)]
#![no_main]

use core::{future::Future, pin::Pin, task::{Context, Poll}};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct Inline;

impl Future for Inline {
    type Output = i32;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(42i32)
    }
}

pub async gen fn foo() -> i32 {
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

pub async gen fn foo2() -> i32 {
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
