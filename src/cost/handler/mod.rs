use crate::config_file::model::NodeCost;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CostCalulator {
    node_cost: NodeCost,
    cpu: f32,
    memory: f32,
    scale: i32,
}

impl CostCalulator {

    pub fn new() -> CostCalulator {
        Default::default()
    }

    pub fn from(node_cost: NodeCost, cpu: f32, memory: f32, scale: i32) -> CostCalulator {
        CostCalulator {
            node_cost,
            cpu,
            memory,
            scale
        }
    }

    pub fn hourly(&mut self) -> f32 {
        let cost:f32 = (self.monthly_cost() / 30 as f32) / 24 as f32;

        cost * self.scale as f32
    }

    pub fn daily(&mut self) -> f32 {
        let cost:f32 = self.monthly_cost() / (30 as f32);

        cost * self.scale as f32
    }

    pub fn weekly(&mut self) -> f32 {
        let cost:f32 = (self.monthly_cost() / 30 as f32) * 7 as f32;

        cost * self.scale as f32
    }

    pub fn monthly(&mut self) -> f32 {
        let cost:f32 = self.monthly_cost();

        cost * self.scale as f32
    }

    pub fn monthly_cost(&mut self) -> f32 {
        let mut cost:f32 = 0.0;
        let cpu_usage_base = self.node_cost.cpu/self.cpu;
        let memory_usage_base = self.node_cost.memory/self.memory;
        // First let verify the request can actualy fit on the requested node type.  If it does
        // we can use the highest cost based on cpu or memory( no reason to consider both atm).
        if cpu_usage_base >= 1.0 && memory_usage_base >= 1.0 {
            let cpu_cost_base: f32 = self.node_cost.price/(self.node_cost.cpu as f32) * self.cpu as f32;
            let memory_cost_base: f32 = self.node_cost.price/(self.node_cost.memory as f32) * self.memory as f32;
            cost = cpu_cost_base.max(memory_cost_base);
        }

        cost * self.scale as f32
    }
}
