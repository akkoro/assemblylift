//! The Threader Runtime
//! "Threader" is the interface between the Wasmtime runtime and the IOmod RPC network.
//! See [core-threader doc](../../docs/core-threader.md) for more details.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;

use assemblylift_core_iomod::registry::{RegistryChannelMessage, RegistryTx};

use super::buffers::IoBuffer;
use super::wasm::asml_io;

pub type IoId = u32;

pub struct Threader<S> {
    io_memory: Arc<Mutex<IoMemory>>,
    registry_tx: RegistryTx,
    _phantom: std::marker::PhantomData<S>,
}

impl<S> Threader<S>
where
    S: Clone + Send + Sized + 'static,
{
    /// Create a new Threader instance with the provided sender `tx`
    pub fn new(tx: RegistryTx) -> Self {
        Threader {
            io_memory: Arc::new(Mutex::new(IoMemory::new())),
            registry_tx: tx,
            _phantom: std::marker::PhantomData::default(),
        }
    }

    /// Issue an unused IOID for a new IOmod call
    pub fn next_ioid(&mut self) -> Option<IoId> {
        match self.io_memory.clone().lock() {
            Ok(mut memory) => memory.next_id(),
            Err(_) => None,
        }
    }

    /// Poll the runtime for the completion status of call associated with `ioid`
    pub fn poll(&mut self, ioid: IoId) -> Option<Vec<u8>> {
        match self.io_memory.clone().lock() {
            Ok(memory) => match memory.poll(ioid) {
                true => Some(memory.buffer.get(ioid as usize)),
                false => None,
            },
            Err(_) => None,
        }
    }

    /// Invoke the IOmod call at `method_path` with `method_input`, and assign it id `ioid`.
    /// A task is spawned on the tokio runtime which runs until the IOmod call responds.
    pub fn invoke(
        &mut self,
        method_path: &str,
        method_input: Vec<u8>,
        ioid: IoId,
    ) -> Result<(), asml_io::IoError> {
        let io_memory = self.io_memory.clone();

        let coords = method_path.split(".").collect::<Vec<&str>>();
        if coords.len() != 4 {
            tracing::error!("io invoke failed: malformed module coordinates");
            return Err(asml_io::IoError::InvalidCoords);
        }

        let iomod_coords = format!("{}.{}.{}", coords[0], coords[1], coords[2]);
        let method_name = format!("{}", coords[3]);

        let registry_tx = self.registry_tx.clone();
        let (local_tx, mut local_rx) = mpsc::channel(100);

        tokio::spawn(async move {
            registry_tx
                .send(RegistryChannelMessage {
                    iomod_coords,
                    method_name,
                    payload_type: "IOMOD_REQUEST",
                    payload: method_input,
                    responder: Some(local_tx.clone()),
                })
                .await
                .unwrap();
        });

        tokio::spawn(async move {
            if let Some(response) = local_rx.recv().await {
                io_memory
                    .lock()
                    .unwrap()
                    .handle_response(response.payload, ioid);
            }
        });

        Ok(())
    }
}

struct IoMemory {
    next_id: IoId,
    buffer: IoBuffer,
    io_status: HashMap<IoId, bool>,
}

impl IoMemory {
    fn new() -> Self {
        IoMemory {
            next_id: 1, // id 0 is reserved (null)
            buffer: IoBuffer::new(),
            io_status: Default::default(),
        }
    }

    fn next_id(&mut self) -> Option<IoId> {
        let next_id = self.next_id.clone();
        self.next_id += 1;
        self.io_status.insert(next_id, false);
        Some(next_id)
    }

    fn poll(&self, ioid: IoId) -> bool {
        match self.io_status.get(&ioid) {
            Some(status) => *status,
            None => false,
        }
    }

    fn handle_response(&mut self, response: Vec<u8>, ioid: IoId) {
        self.buffer.set(ioid as usize, response.clone());
        self.io_status.insert(ioid, true);
    }
}
