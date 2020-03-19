pub mod model;
pub mod handler;

use crate::config_file::ConfigurationControl;
use crate::command_control::cmd_model::predict::{PredictCost};
pub use model::*;
pub use handler::*;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CostRunner {}

impl CostRunner {
    pub async fn run(config: &ConfigurationControl, request: &PredictCost) -> Result<CostItem, ()>{
        let mut cost_item = CostItem::new();
        let node_type = config.clone().get_node_cost(request.node_type.clone());
        let mut calculation = handler::CostCalulator::from(node_type, request.cpu, request.memory, request.scale);

        cost_item.node_type = Some(request.node_type.clone());
        cost_item.cpu = Some(request.cpu);
        cost_item.memory = Some(request.memory);
        cost_item.monthly = Some(calculation.monthly());
        cost_item.weekly = Some(calculation.weekly());
        cost_item.daily = Some(calculation.daily());
        cost_item.hourly = Some(calculation.hourly());
        cost_item.scale = Some(request.scale);

        Ok(cost_item)
    }
}
