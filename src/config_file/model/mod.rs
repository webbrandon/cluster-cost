pub mod node_cost;
pub use node_cost::NodeCost;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationAPI {
    kind: String,
    version: String,
    pub specs: ConfigurationSpec,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all="camelCase")]
pub struct ConfigurationSpec {
    pub node_types: BTreeMap<String, NodeCost>,
}

impl ConfigurationAPI {

    pub fn new() -> ConfigurationAPI {
        let mut default = BTreeMap::new();
        default.insert("default".to_string(), NodeCost::new());
        ConfigurationAPI::from_values(
            ConfigurationSpec::from_values(
                default
            )
        )
    }

    pub fn from_values(spec: ConfigurationSpec) -> ConfigurationAPI {
        ConfigurationAPI {
            kind: String::from("config"),
            version: String::from("alpha/1.0"),
            specs: spec,
        }
    }

}

impl ConfigurationSpec {

    pub fn from_values(node_types: BTreeMap<String, NodeCost>) -> ConfigurationSpec {
        ConfigurationSpec {
            node_types,
        }
    }

}
