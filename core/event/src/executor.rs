use std::collections::HashMap;
use std::future::Future;
use std::ops::{Deref, DerefMut};

use futures::{FutureExt, TryFutureExt};
use indexmap::map::IndexMap;

use {
    futures::{
        future::BoxFuture,
        task::{ArcWake, Context, waker_ref}
    },
    std::{
        sync::{Arc, Mutex},
        sync::mpsc::{Receiver, sync_channel, SyncSender},
        task::Poll
    }
};

use crate::constants::EVENT_BUFFER_SIZE_BYTES;
use crate::Event;
use crossbeam_utils::atomic::AtomicCell;

pub struct Executor {
    ready_queue: Arc<Mutex<Receiver<Arc<Task>>>>,
    spawner: Spawner,
    memory: ExecutorMemory
}

impl Executor {
    pub fn new() -> Self {
        let (task_sender, ready_queue) = sync_channel(10_000);

        Executor {
            ready_queue: Arc::new(Mutex::new(ready_queue)),
            spawner: Spawner { task_sender },
            memory: ExecutorMemory::new()
        }
    }

    pub fn run(&mut self) {
        while let Ok(task) = (&*self.ready_queue).lock().unwrap().recv() {
            println!("Got task with event_id: {}", task.event_id);

            if let Ok(mut guarded_future) = task.future.lock() {
                if let Some(mut future) = guarded_future.take() {
                    let waker = waker_ref(&task);
                    let context  = &mut Context::from_waker(&*waker);

                    println!("Polling {}...", task.event_id);
                    if let Poll::Pending = future.as_mut().poll(context) {
                        println!("TRACE: future still pending");
                        *guarded_future = Some(future);
                    } else {
                        println!("TRACE: future ready")
                    }
                }
            }
        }
    }

    pub fn next_event_id(&mut self) -> Option<u32> {
        self.memory.next_id()
    }

    pub fn spawn_with_event_id(&self, writer: Arc<*const AtomicCell<u8>>, future: impl Future<Output=Vec<u8>> + 'static + Send, event_id: u32) {
        println!("TRACE: spawn_with_event_id");

        // clone is fine, as long as we're sure that the addresses aren't stale
        // TODO not sure of performance of clone here though
        let memory = self.memory.clone();

        let mut wr = writer.clone();
        // FIXME this is suuuuper kludgy
        let slc = unsafe { std::slice::from_raw_parts(*wr, EVENT_BUFFER_SIZE_BYTES) };
        let with_writer = async move {
            println!("TRACE: awaiting IO...");
            let serialized = future.await;
            memory.write_vec_at(slc, serialized, event_id);
            println!("TRACE: wrote to WASM memory");
        };

        self.spawner.spawn(with_writer, event_id)
    }
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output=()> + 'static + Send, event_id: u32) {
        println!("TRACE: spawn");

        let boxed_future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(boxed_future)),
            task_sender: self.task_sender.clone(),
            event_id
        });

        self.task_sender.send(task).expect("too many tasks already queued") // MUSTDO better error handling
    }
}

pub struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
    event_id: u32
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks already queued") // MUSTDO better error handling
    }
}

#[derive(Clone)]
struct Document {
    start: usize,
    length: usize
}

#[derive(Clone)]
struct ExecutorMemory {
    _next_id: u32,
    document_map: IndexMap<usize, Document>
}

impl ExecutorMemory {
    pub fn new() -> Self {
        ExecutorMemory {
            _next_id: 0,
            document_map: Default::default()
        }
    }

    pub fn next_id(&mut self) -> Option<u32> {
        let next_id = self._next_id.clone();
        self._next_id += 1;

        Some(next_id)
    }

    pub fn write_vec_at(&self, writer: &[AtomicCell<u8>], vec: Vec<u8>, event_id: u32) {
        println!("TRACE: write_vec_at");

        let index = event_id as usize;
        let required_length = vec.len();

        let start = self.find_with_length(required_length);
        let end = start + required_length;
        for i in start..end {
            writer[i].store(vec[i - start]);
        }
    }

    fn find_with_length(&self, length: usize) -> usize {
        0usize
    }
}
