use core::ops::{Coroutine, CoroutineState};
use core::pin::Pin;

fn as_u8(v: u8) {}
fn as_u16(v: u16) {}
fn test() {
    let mut coroutine = |r| {
        let a = yield 0;
        let a = yield 1;
        let a = yield 2;
        3
    };

    match Pin::new(&mut coroutine).resume(0i8) {
        CoroutineState::Yielded(yielded) => { as_u8(yielded); }
        CoroutineState::Complete(return_value) => { as_u16(return_value); }
    }
}