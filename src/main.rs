#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_ui;
mod components;
mod k8s;
mod tokio_runtime_wrapper;

use crate::app_ui::AppUi;

use crate::k8s::KubeConnection;
use crate::tokio_runtime_wrapper::TokioRuntime;
use anyhow::Result;

fn main() -> Result<()> {
    // init rustls
    debug_assert!(
        rustls::crypto::ring::default_provider()
            .install_default()
            .is_ok(),
        "rustls has been already initialized"
    );

    // init tokio runtime and attach to main thread
    // tokio is axillary rt here, so no `#[tokio::main]`
    let rt = TokioRuntime::new()?;
    let _rt_ctx = rt.enter();

    // init k8s client
    // TODO: later refactor to a connection manager
    //   to manage connections and secrets
    let kube_client = KubeConnection::new()?;

    // TODO: init is blocking. need to detach init to async
    //   task and show spinner at the app root
    //   and handle errors in ui (red text or kinda)

    // create and launch ui window + gpui/smol rt
    AppUi::launch(kube_client, rt.handle());

    Ok(())
}
