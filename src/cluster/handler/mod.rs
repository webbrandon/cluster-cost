use crate::toolbelt::CommandLineHandler;
use super::model::*;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterHandler {}

impl ClusterHandler {
    pub async fn run(ns: &Option<String>, dry_run: bool, verbose: bool) -> KubeDeployments {
        let mut cmd_handler = CommandLineHandler::new();
        let mut deployments_cmd = "kubectl get deployment ".to_string();
        
        match &ns {
            Some(namespace) => {
                deployments_cmd.push_str("--namespace ");
                deployments_cmd.push_str(namespace);
            },
            None => {
                deployments_cmd.push_str("--all-namespaces");
            }
        }
        deployments_cmd.push_str(" -o json");

        let deployments_string = cmd_handler.run_cmd(&deployments_cmd, dry_run, verbose);
        let deployments: KubeDeployments = serde_json::from_str(&deployments_string).unwrap();

        deployments
    }
}
