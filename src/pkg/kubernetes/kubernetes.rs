use k8s_openapi::api::apps::v1::{Deployment};
use kube::{Api, Client};
use kube::api::{GetParams, Patch, PatchParams};
use chrono::{DateTime, Local, SecondsFormat};
use serde_json::json;

pub struct KubernetesClient {
    client: Client
}

#[allow(unused)]
pub fn now_with_rfc3339() -> String {
    let dt = Local::now();
    let naive_utc = dt.naive_utc();
    let offset = dt.offset().clone();
    let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
    dt_new.to_rfc3339_opts(SecondsFormat::Secs, true)
}

impl KubernetesClient {
    #[allow(unused)]
    pub async fn new() -> Result<KubernetesClient, Box<dyn std::error::Error>> {
        let client = Client::try_default().await?;
        Ok(KubernetesClient{
            client
        })
    }

    #[allow(unused)]
    pub async fn rollout_restart(&self, name: &str, namespace: &str) -> Result<Deployment, Box<dyn std::error::Error>> {
        let deploys: Api<Deployment> = Api::namespaced(self.client.clone(), namespace);
        let deploy = deploys.get_with(name, &GetParams::default()).await?;
        let patch = json!({
                "spec": {
                    "template": {
                        "metadata": {
                            "annotations": {
                                "kubectl.kubernetes.io/restartedAt": now_with_rfc3339()
                            }
                        }
                    }
                }
            });
        let pp = PatchParams::default();
        Ok(deploys.patch(name, &pp, &Patch::Merge(&patch)).await?)
    }
}



#[cfg(test)]
mod test {
    use tokio_test;
    use super::*;

    #[test]
    fn test_fetch_uri() {
        tokio_test::block_on(async {
            let client = KubernetesClient::new().await.unwrap();
            let result = client.rollout_restart("ks-apiserver", "kubesphere-system").await.unwrap();
            let result = client.rollout_restart("ks-controller-manager", "kubesphere-system").await.unwrap();
            println!("{:?}", result);
        });
    }


}
