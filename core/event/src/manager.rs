use std::collections::HashMap;
use std::sync::Mutex;
use std::any::*;
use crate::Event;

use indexmap::IndexMap;
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};
use std::ops::{Deref, DerefMut};

pub type DynFut<T> = Pin<Box<dyn ::std::future::Future<Output = T>>>;
type StoredFut = Box<dyn Fn() -> DynFut<Vec<u8>>>;

pub struct EventManager {
    futures: IndexMap<u32, StoredFut>
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            futures: Default::default()
        }
    }

    pub fn add(&mut self, future: StoredFut) -> u32 {
        let event_id = 0;
        self.futures.entry(event_id).or_insert(future);

        event_id
    }

    pub fn get(&self, event_id: u32) -> DynFut<Vec<u8>> {
        (self.futures.get(&event_id).unwrap())()
    }

    pub async fn run(&mut self, event_id: u32) -> Vec<u8> {
        (self.futures.get_mut(&event_id).unwrap().deref_mut())().await
    }
}
