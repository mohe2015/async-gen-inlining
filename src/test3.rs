use std::{pin::Pin, ops::CoroutineState};
use std::ops::Coroutine;

pub fn take_i64(_input: i64) {

}

pub fn test() {
    let mut coroutine = || {
        yield 1;
    };
    match Pin::new(&mut coroutine).resume(()) {
        CoroutineState::Yielded(test) => {}
        _ => panic!("unexpected value from resume"),
    }
}