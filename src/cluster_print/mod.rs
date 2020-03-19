use crate::cost::model::CostItem;
use crate::toolbelt::TablePrint;

use prettytable::{Table, Row};


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterPrintRunner {}

impl ClusterPrintRunner {
    pub async fn run(cluster: &[CostItem]) -> Result<(), ()> {
        let mut rows: Vec<Row> = Vec::new();
        let mut table = Table::new();
        table.add_row(row![Frb => "Namespace", "Deployment", "CPU", "MEM", "Scale", "Month", "Week", "Day", "Hour"]);

        for deploy in cluster {
            let namespace = if deploy.namespace.is_some() {deploy.namespace.to_owned().unwrap()} else {"".to_string()};
            let deployment = if deploy.deployment.is_some() {deploy.deployment.to_owned().unwrap()} else {"".to_string()};
            let cpu = if deploy.cpu.is_some() {deploy.cpu.to_owned().unwrap()} else {0.0};
            let memory = if deploy.memory.is_some() {deploy.memory.to_owned().unwrap()} else {0.0};
            let scale = if deploy.scale.is_some() {deploy.scale.to_owned().unwrap()} else {0};
            let monthly = if deploy.monthly.is_some() {deploy.monthly.to_owned().unwrap()} else {0.0};
            let weekly = if deploy.weekly.is_some() {deploy.weekly.to_owned().unwrap()} else {0.0};
            let daily = if deploy.daily.is_some() {deploy.daily.to_owned().unwrap()} else {0.0};
            let hourly = if deploy.hourly.is_some() {deploy.hourly.to_owned().unwrap()} else {0.0};
            if scale > 0 {
                rows.push(row![namespace, deployment, cpu, memory, scale, monthly, weekly, daily, hourly]);
            }
        }

        TablePrint::new().print(table, rows);

        Ok(())
    }
}
