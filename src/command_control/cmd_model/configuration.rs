use structopt::StructOpt;
use super::completions::Completions;

#[derive(StructOpt, Debug, Clone)]
pub enum Configurations {
    /// Set configuration file value to something new.
    #[structopt(name = "set")]
    AppConfiguration(AppConfiguration),

    /// Completion scripts for various shells.
    #[structopt(name = "completions")]
    Completions(Completions),
}

#[derive(StructOpt, Debug, Default, Clone)]
pub struct AppConfiguration {

    /// Perform action on node type config. (create, read, update, delete)
    pub action: Option<String>,

    /// Field being targeted for change.
    pub node_type: Option<String>,

    /// Target field new value.
    pub price: Option<i32>,

    /// Target field new value.
    pub cpu: Option<i32>,

    /// Target field new value.
    pub memory: Option<i32>,
}
