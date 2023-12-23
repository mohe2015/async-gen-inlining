#![feature(gen_blocks, async_iterator, coroutines, coroutine_trait, noop_waker)]

use std::async_iter::AsyncIterator;
use std::pin::pin;
use std::task::{Context, Waker, Poll};
use std::{pin::Pin, ops::CoroutineState};
use std::ops::Coroutine;

pub fn take_iterator(_iterator: impl Iterator<Item = i32>) {

}

pub fn test() {
    let mut coroutine = |e: i32| {
        yield e + 1;
        yield e + 3;
        "hi"
    };

    match Pin::new(&mut coroutine).resume(1) {
        CoroutineState::Yielded(1) => {}
        _ => panic!("unexpected value from resume"),
    }

    let iterator = gen {
        yield 1;
    };
    take_iterator(iterator);

    let async_ = async {
        "hi"
    };

    let async_gen = async gen {
        yield "hi";
    };
    
    let waker = Waker::noop();
    let mut context = Context::from_waker(&waker);

    match pin!(async_gen).poll_next(&mut context) {
        Poll::Ready(value) => {}
        Poll::Pending => {}
    }
}