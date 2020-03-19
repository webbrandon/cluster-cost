pub mod model;
pub mod handler;
pub mod run;

use crate::command_control::cmd_model::analyze::{AnalyzeCluster};
pub use model::*;
pub use handler::*;
pub use run::*;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterRunner {}

impl ClusterRunner {
    pub async fn run(request: &AnalyzeCluster) -> Result<Vec<ClusterDetail>, ()> {
        let dry_run = request.dry_run;
        let verbose = request.verbose;
        let mut data: Vec<ClusterDetail> = Vec::new();
        let deployments = handler::ClusterHandler::run(&request.namespace, dry_run, verbose).await;
        match deployments.items {
            Some(list) => {
                for deploy in &list {
                    let mut details = ClusterDetail::new();
                    details.from_deployment(deploy.to_owned());
                    data.push(details);
                }
            },
            None => {
                ()
            },
        }

        Ok(data)
    }
}
