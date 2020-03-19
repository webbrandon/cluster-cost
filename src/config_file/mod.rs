pub mod model;
pub mod handler;

use crate::toolbelt::file_handler::ReadFile;

use std::env;
use std::path::Path;
use dirs;

pub use model::{
    ConfigurationAPI
};
pub use model::node_cost::{
    NodeCost,
};
pub use handler::{
    ConfigurationHandler
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationControl {
    pub config: ConfigurationAPI,
}

impl ConfigurationControl {

    pub fn new() -> ConfigurationControl {
        Default::default()
    }

    pub fn load(mut self) -> ConfigurationControl {
        let config_file = self.load_config("config.yaml");

        self.config = ConfigurationHandler::load_manager_config(&config_file);

        self
    }

    pub fn get_node_cost(&mut self, key: String) -> NodeCost {
        match self.config.specs.node_types.get(&key) {
            Some(cost) => cost.to_owned(),
            none => NodeCost::new(),
        }
    }

    pub fn load_config(&mut self, config: &str) -> String {
        let reader = ReadFile::new();
        let mut home = match dirs::home_dir() {
            Some(path) => path,
            None => Path::new(&std::env::var("CLUSTER_COST_CONFIG_BASE").expect("~/")).to_path_buf(),
        };
        home.push(".cluster_cost");
        home.push(config);
        reader.load(
            home
        )
    }

}
