use structopt::StructOpt;

#[derive(StructOpt, Debug, Default, Clone)]
pub struct PredictCost {
    /// The node type used for calculation.
    #[structopt(short = "n", long = "node-type")]
    pub node_type: String,

    /// CPU requirement.
    #[structopt(short = "c", long = "cpu")]
    pub cpu: f32,

    /// Cluster target.
    #[structopt(short = "m", long = "memory")]
    pub memory: f32,

    /// Cluster target.
    #[structopt(short = "s", long = "scale", default_value = "1")]
    pub scale: i32,
}
