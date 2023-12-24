use core::task::{Context, Poll};
use core::async_iter::AsyncIterator;

fn f(v: i64) {}
fn test(mut context: Context) {
    let mut g = async gen {
        let a = yield 0;
        let a = yield 1;
        let a = yield 2;
    };

    match g.poll_next(&mut context) {
        Poll::Ready(y) => { f(y); } // FIXME test with 1 there I think this infers stuff it should not
        Poll::Pending => {}
    }
}