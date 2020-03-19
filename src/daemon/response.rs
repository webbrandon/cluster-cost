use crate::cost::CostItem;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterCostResponse {
    pub data: ClusterCostData,
    pub error: Option<String>,
}

impl ClusterCostResponse {

    pub fn new() -> ClusterCostResponse {
        Default::default()
    }

    pub fn set_attributes(&mut self, val: CostItem) {
        self.data.set_attributes(val);
    }

    pub fn set_error(&mut self, val: String) {
        self.error = Some(val);
    }

}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterCostData {
    pub attributes: CostItem,
}

impl ClusterCostData {

    pub fn set_attributes(&mut self, val: CostItem) {
        self.attributes = val;
    }

}
