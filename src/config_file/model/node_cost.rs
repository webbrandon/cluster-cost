#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NodeCost {
    pub price: f32,
    pub cpu: f32,
    pub memory: f32,
}

impl NodeCost {
    pub fn new() -> NodeCost {
        Default::default()
    }

    pub fn from_values(price: Option<f32>, cpu: Option<f32>, memory: Option<f32>) -> NodeCost {
        let price: f32 = if price.is_some() {price.unwrap()} else {0.0};
        let cpu: f32 = if cpu.is_some() {cpu.unwrap()} else {0.0};
        let memory: f32 = if memory.is_some() {memory.unwrap()} else {0.0};
        NodeCost {
            price,
            cpu,
            memory,
        }
    }

}
