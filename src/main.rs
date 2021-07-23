use kube::{api::{Api, ListParams, ResourceExt, DeleteParams}, Client};
use k8s_openapi::api::core::v1::Pod;
use log::{info, debug};
use env_var::env_var;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    env_logger::init();
    let ns = env_var!(required "NAMESPACE", default:"default");
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(client, &ns);
    let lp = ListParams::default().fields("status.phase=Failed");
    let dp = DeleteParams::default();
    match pods.delete_collection(&dp, &lp).await? {
        either::Left(list) => {
            let names: Vec<_> = list.iter().map(ResourceExt::name).collect();
            debug!("Deleting collection of pods: {:?}", names);
        },
        either::Right(status) => {
            info!("Deleted collection of pods: status={:?}", status);
        }
    }
    Ok(())
}
