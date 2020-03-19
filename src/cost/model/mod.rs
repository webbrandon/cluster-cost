// This struct describes cost of a ClusterDeployment type.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CostItem {
    pub node_type: Option<String>,
    pub namespace: Option<String>,
    pub deployment: Option<String>,
    pub scale: Option<i32>,
    pub cpu: Option<f32>,
    pub memory: Option<f32>,
    pub hourly: Option<f32>,
    pub daily: Option<f32>,
    pub weekly: Option<f32>,
    pub monthly: Option<f32>,
}

impl CostItem {
    pub fn new() -> CostItem {
        Default::default()
    }
}
