#![feature(gen_blocks, async_iterator, coroutines, coroutine_trait)]

use std::{pin::Pin, ops::CoroutineState};
use std::ops::Coroutine;

pub fn take_iterator(iterator: impl Iterator<Item = i32>) {

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
}