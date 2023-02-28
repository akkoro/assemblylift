use bitflags::bitflags;

use crate::clocks::WasiMonotonicClock;
use crate::stream::{InputStream, OutputStream};
use crate::Error;

bitflags! {
    pub struct RwEventFlags: u32 {
        const HANGUP = 0b1;
    }
}

pub enum RwStream<'a> {
    Read(&'a dyn InputStream),
    Write(&'a dyn OutputStream),
}

pub struct RwSubscription<'a> {
    pub stream: RwStream<'a>,
    status: Option<Result<RwEventFlags, Error>>,
}

impl<'a> RwSubscription<'a> {
    pub fn new_input(stream: &'a dyn InputStream) -> Self {
        Self {
            stream: RwStream::Read(stream),
            status: None,
        }
    }
    pub fn new_output(stream: &'a dyn OutputStream) -> Self {
        Self {
            stream: RwStream::Write(stream),
            status: None,
        }
    }
    pub fn complete(&mut self, flags: RwEventFlags) {
        self.status = Some(Ok(flags))
    }
    pub fn error(&mut self, error: Error) {
        self.status = Some(Err(error))
    }
    pub fn result(&mut self) -> Option<Result<RwEventFlags, Error>> {
        self.status.take()
    }
    pub fn is_complete(&self) -> bool {
        self.status.is_some()
    }
}

pub struct MonotonicClockSubscription<'a> {
    pub clock: &'a dyn WasiMonotonicClock,
    pub deadline: u64,
}

impl<'a> MonotonicClockSubscription<'a> {
    pub fn now(&self) -> u64 {
        self.clock.now()
    }
    pub fn duration_until(&self) -> Option<u64> {
        self.deadline.checked_sub(self.now())
    }
    pub fn result(&self) -> Option<Result<(), Error>> {
        if self.now().checked_sub(self.deadline).is_some() {
            Some(Ok(()))
        } else {
            None
        }
    }
}

pub enum Subscription<'a> {
    ReadWrite(RwSubscription<'a>),
    MonotonicClock(MonotonicClockSubscription<'a>),
}

#[derive(Debug)]
pub enum SubscriptionResult {
    ReadWrite(Result<RwEventFlags, Error>),
    MonotonicClock(Result<(), Error>),
}

impl SubscriptionResult {
    pub fn from_subscription(s: Subscription) -> Option<SubscriptionResult> {
        match s {
            Subscription::ReadWrite(mut s) => {
                s.result().map(|sub| SubscriptionResult::ReadWrite(sub))
            }
            Subscription::MonotonicClock(s) => s.result().map(SubscriptionResult::MonotonicClock),
        }
    }
}
