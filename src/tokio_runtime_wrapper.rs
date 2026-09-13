use gpui_kit::Global;
use tokio::runtime::{Builder, EnterGuard, Handle, Runtime};
use tokio::task::JoinHandle;

/// Tokio runtime owner struct
pub struct TokioRuntime {
    _rt: Runtime, // keep runtime alive
    handle: Handle,
}

impl TokioRuntime {
    pub fn new() -> std::io::Result<Self> {
        let rt = Builder::new_multi_thread().enable_all().build()?;
        let handle = rt.handle().clone();
        Ok(Self { _rt: rt, handle })
    }

    pub fn enter(&self) -> EnterGuard<'_> {
        self.handle.enter()
    }

    pub fn handle(&self) -> TokioRuntimeHandle {
        TokioRuntimeHandle(self.handle.clone())
    }
}

/// gpui injectable tokio runtime handle to spawn tasks in tokio rt
#[derive(Clone)]
pub struct TokioRuntimeHandle(pub Handle);

impl TokioRuntimeHandle {
    /// Proxy call for [`Handle::spawn`]
    #[inline]
    #[track_caller]
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        // signature copied from `Handle::spawn`
        self.0.spawn(future)
    }
}
impl Global for TokioRuntimeHandle {}
