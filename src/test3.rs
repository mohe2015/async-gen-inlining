use std::pin::Pin;
use std::ops::Coroutine;

pub fn test() {
    let mut coroutine = || {
        yield 1i32;
    };
    let _result = Pin::new(&mut coroutine).resume(());
}