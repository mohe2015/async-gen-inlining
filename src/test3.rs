use std::{pin::Pin, ops::CoroutineState};
use std::ops::Coroutine;

fn take_i64(input: i64) {

}

fn test() {
    let coroutine = || {
        yield 1;
    };
    match Pin::new(&mut coroutine).resume(()) {
        CoroutineState::Yielded(test) => {take_i64(test)}
        _ => panic!("unexpected value from resume"),
        CoroutineState::Complete(_) => todo!(),
    }
    
}