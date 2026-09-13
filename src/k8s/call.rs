use k8s_openapi::api::core::v1::Namespace;
use kube::Api;
use kube::api::ListParams;

pub async fn list_namespaces(client: kube::Client) -> kube::Result<Vec<Namespace>> {
    let api: Api<Namespace> = Api::all(client);
    let list = api.list(&ListParams::default()).await?;
    let items = list.items;

    Ok(items)
}
