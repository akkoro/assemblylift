use std::future::Future;
use std::ops::DerefMut;
use std::pin::Pin;

use futures::FutureExt;

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

use crate::Event;
use crate::constants::EVENT_BUFFER_SIZE_BYTES;

pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
    spawner: Spawner,
    memory: ExecutorMemory
}

impl Executor {
    pub fn new() -> Self {
        let (task_sender, ready_queue) = sync_channel(10_000);

        Executor {
            ready_queue,
            spawner: Spawner { task_sender },
            memory: ExecutorMemory {}
        }
    }

    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            if let Ok(mut guarded_future) = task.future.lock() {
                if let Some(mut future) = guarded_future.take() {
                    let waker = waker_ref(&task);
                    let context  = &mut Context::from_waker(&*waker);

                    if let Poll::Pending = future.as_mut().poll(context) {
                        *guarded_future = Some(future);
                    }
                }
            }
        }
    }

    pub fn make_event(&self) -> Option<Event> {
        if let Some(event_id) = self.memory.find_next_block() {
            return Some(Event::new(event_id));
        }

        None
    }

    pub fn spawn_as_event(&self, future: impl Future<Output=Vec<u8>> + 'static + Send, event: &Event) {
            self.spawner.spawn(future, event.id)
    }
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output=Vec<u8>> + 'static + Send, event_id: u32) {
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
    future: Mutex<Option<BoxFuture<'static, Vec<u8>>>>,
    task_sender: SyncSender<Arc<Task>>,
    event_id: u32
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks already queued") // MUSTDO better error handling
    }
}

struct ExecutorMemory {}

impl ExecutorMemory {
    pub fn find_next_block(&self) -> Option<u32> {
        // event id corresponds to _starting_ block
        // each block may need a small header
        None
    }
}
