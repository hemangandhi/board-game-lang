/// A Set of utilities for the stream handling in the server.
/// Not intended for more general use.
use std::clone::Clone;
use std::mem;
use std::sync::Mutex;

use core::pin::Pin;
use futures_util::{sink::Sink, task::Context, task::Poll};

// Private utilities to foldM over slices of Poll<Result<T, E>>
// or Result<T, E>.
fn poll_and_then<T, U, F>(p: Poll<T>, f: &mut F) -> Poll<U>
where
    F: FnMut(T) -> Poll<U>,
{
    match p {
        Poll::Ready(r) => f(r),
        Poll::Pending => Poll::Pending,
    }
}

fn fold_poll_results<F, CI, I, E>(
    container: &mut [Option<CI>],
    result_map: &mut F,
    default: I,
) -> Poll<Result<I, E>>
where
    F: FnMut(Pin<&mut CI>) -> Poll<Result<I, E>>,
    CI: Unpin,
{
    container.iter_mut().filter_map(|x| x.as_mut()).fold(
        Poll::Ready(Ok(default)),
        &mut |acc: Poll<Result<I, E>>, mut ci: &mut CI| -> Poll<Result<I, E>> {
            poll_and_then(acc, &mut |pr| match pr {
                Ok(_) => {
                    let pinned: Pin<&mut CI> = unsafe { Pin::new_unchecked(&mut ci) };
                    result_map(pinned)
                }
                Err(e) => Poll::Ready(Err(e)),
            })
        },
    )
}

fn fold_results<F, CI, I, E>(
    container: &mut [Option<CI>],
    result_map: &mut F,
    default: I,
) -> Result<I, E>
where
    F: FnMut(Pin<&mut CI>) -> Result<I, E>,
    CI: Unpin,
{
    container
        .iter_mut()
        .filter_map(|x| x.as_mut())
        .fold(Ok(default), &mut |acc: Result<I, E>,
                                 mut r: &mut CI|
         -> Result<I, E> {
            acc.and_then(|_| {
                let pinned: Pin<&mut CI> = unsafe { Pin::new_unchecked(&mut r) };
                result_map(pinned)
            })
        })
}

/// A Vec of Sinks over an Item.
/// We assume that the Sink<Item> is Unpin for most of the usage.
/// This struct should be thread safe.
///
/// The underlying Vec uses an Option to mark deletion so that individual
/// sinks can be closed at different times. This is done with the following
/// 2-step invariant:
///   1. Upon insertion, a usize is returned as a key for the inserted sink.
///   2. A sink is retrived (hence removed) when its key is provided.
pub struct VecSink<SinkType> {
    sinks: Mutex<Vec<Option<SinkType>>>,
}

impl<S: Unpin> Unpin for VecSink<S> {}

/// An implementation of the VecSink to be a Sink.
/// Sends clonable items to each Sink in the vector. This should be thread-safe,
/// but could result in partial sends. The returned Poll<Result<...>> is
/// Ready(Ok()) when all the underlying sinks return Ready(Ok()). The first
/// Pending or Ready(Error(...)) returned value is otherwise returned and none
/// of the other sinks have the relevant function invoked on them.
impl<SinkType, Item> Sink<Item> for VecSink<SinkType>
where
    Item: Clone,
    SinkType: Sink<Item> + Unpin,
{
    type Error = SinkType::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        fold_poll_results(
            &mut self.get_mut().sinks.lock().unwrap(),
            &mut |s: Pin<&mut SinkType>| s.poll_ready(cx),
            (),
        )
    }

    fn start_send(self: Pin<&mut Self>, i: Item) -> Result<(), Self::Error> {
        fold_results(
            &mut self.get_mut().sinks.lock().unwrap(),
            &mut |s: Pin<&mut SinkType>| s.start_send(i.clone()),
            (),
        )
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        fold_poll_results(
            &mut self.get_mut().sinks.lock().unwrap(),
            &mut |s: Pin<&mut SinkType>| s.poll_flush(cx),
            (),
        )
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        fold_poll_results(
            &mut self.get_mut().sinks.lock().unwrap(),
            &mut |s: Pin<&mut SinkType>| s.poll_close(cx),
            (),
        )
    }
}

impl<S> VecSink<S> {
    /// An empty sink that will just absorb items into the void.
    pub fn new() -> Self {
        VecSink {
            sinks: Mutex::new(vec![]),
        }
    }

    /// Adds a new sink, returning a stable index to it.
    /// Linear in the number of sinks added.
    pub fn insert(&mut self, p: S) -> usize {
        let mut sinks = self.sinks.lock().unwrap();
        for i in 0..sinks.len() {
            if sinks[i].is_none() {
                sinks[i] = Some(p);
                return i;
            }
        }
        sinks.push(Some(p));
        return sinks.len() - 1;
    }

    /// Removes the indicated sink if possible, returning it.
    /// Returns None if there was no sink at the provided position.
    pub fn delete(&mut self, idx: usize) -> Option<S> {
        let mut sinks = self.sinks.lock().unwrap();
        if idx < sinks.len() {
	    return mem::take(&mut sinks[idx]);
        }
        return None;
    }
}
