use crate::cost::model::CostItem;
use crate::cluster::model::ClusterDetail;
use crate::cost;

use crate::config_file::ConfigurationControl;
use crate::command_control::cmd_model::predict::{PredictCost};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterCostRunner {}

impl ClusterCostRunner {
    pub async fn run(config: &ConfigurationControl, cluster: &[ClusterDetail]) -> Result<Vec<CostItem>, ()> {
        let mut cost_items: Vec<CostItem> = Vec::new();
        for deploy in cluster {
            let predict_this = PredictCost {
                node_type: deploy.node_type_value.to_owned(),
                cpu: deploy.reserved_cpu_total as f32,
                memory: deploy.reserved_memory_total as f32,
                scale: deploy.replicas,
            };
            if let Ok(mut cost_item) = cost::CostRunner::run(config, &predict_this).await {
                cost_item.deployment = Some(deploy.name.clone());
                cost_item.namespace = Some(deploy.namespace.clone());
                cost_items.push(cost_item);
            };
        }

        Ok(cost_items)
    }
}
