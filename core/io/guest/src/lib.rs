use std::future::Future;
use std::io::BufReader;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use serde::{de::DeserializeOwned, Deserialize};

#[derive(Clone)]
/// A handle implementing `std::future::Future` for an in-flight IOmod call
pub struct Io<'a, R> {
    pub id: u32,
    waker: Box<Option<Waker>>,
    _phantom: PhantomData<&'a R>,
}

impl<'a, R: Deserialize<'a>> Io<'_, R> {
    pub fn new(id: u32) -> Self {
        Io {
            id,
            waker: Box::new(None),
            _phantom: PhantomData,
        }
    }
}

impl<'a, R> Future for Io<'_, R>
where
    R: DeserializeOwned,
{
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match asml_io::poll(self.id) {
            Ok(res) => Poll::Ready(read_response::<Self::Output>(res).unwrap()),
            _ => {
                self.waker = Box::new(Some(cx.waker().clone()));
                Poll::Pending
            }
        }
    }
}

fn read_response<'a, T>(res: String) -> Option<T>
where
    T: DeserializeOwned,
{
    match serde_json::from_str(&res) {
        Ok(response) => Some(response),
        Err(why) => {
            // console_log(format!("[ERROR] ioid={} {}", id, why.to_string()));
            None
        }
    }
}
