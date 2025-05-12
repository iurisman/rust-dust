mod pure;

fn main() {}
/*
use core::future::poll_fn;
use std::future::Future;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};
use std::pin::Pin;


// Dummy waker implementation for running futures without an async runtime
fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(std::ptr::null(), &VTABLE)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn main() {
    // Define a closure that returns Poll::Ready with a String
    let read_line = |_cx: &mut Context<'_>| Poll::Ready("Hello, World!".to_owned());

    // Wrap the closure with poll_fn to create a future
    let mut read_future = poll_fn(read_line);

    // Set up a dummy context for polling
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    // Pin and poll the future manually
    let mut pinned = unsafe { Pin::new_unchecked(&mut read_future) };
    match pinned.as_mut().poll(&mut cx) {
        Poll::Ready(val) => println!("Future completed with: {}", val),
        Poll::Pending => println!("Future is pending"),
    }
}

*/
