pub trait RuntimeAbi<S> 
where
    S: Clone + Send + Sized + 'static,
{
    fn success(status_tx: crate::wasm::StatusTx<S>, response: Vec<u8>);
    fn failure(status_tx: crate::wasm::StatusTx<S>, response: Vec<u8>);
}
