use anyhow::Result;
use gpui_kit::Global;
use kube::config::Kubeconfig;

pub mod call;

pub struct KubeConnection {
    kube_config: Kubeconfig,
    kube_client: kube::Client,
}

impl KubeConnection {
    pub fn new() -> Result<KubeConnection> {
        let kubeconfig = Kubeconfig::read()?;
        let client = kube::Client::try_from(kubeconfig.clone())?;

        Ok(KubeConnection {
            kube_config: kubeconfig,
            kube_client: client,
        })
    }

    pub fn get_display_cluster_name(&self) -> Option<String> {
        self.kube_config.current_context.to_owned()
    }

    pub fn client(&self) -> kube::Client {
        self.kube_client.clone()
    }
}

impl Global for KubeConnection {}
