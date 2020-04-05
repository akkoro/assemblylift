use {
    futures::{
        future::BoxFuture,
        task::{ArcWake, Context, waker_ref}
    },
    std::{
        sync::{Arc, Mutex},
        sync::mpsc::{Receiver, SyncSender, sync_channel},
        task::Poll
    }
};
use std::future::Future;
use futures::FutureExt;

pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
    pub spawner: Spawner
}

impl Executor {
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
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let boxed_future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(boxed_future)),
            task_sender: self.task_sender.clone()
        });

        self.task_sender.send(task).expect("too many tasks already queued") // MUSTDO better error handling
    }
}

pub struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks already queued") // MUSTDO better error handling
    }
}

pub fn new_executor() -> Executor {
    let (task_sender, ready_queue) = sync_channel(10_000);

    Executor {
        ready_queue,
        spawner: Spawner { task_sender }
    }
}
