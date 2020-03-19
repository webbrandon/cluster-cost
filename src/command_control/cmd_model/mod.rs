use structopt::StructOpt;
use structopt::clap::AppSettings::*;
pub mod cmdctl;
pub mod analyze;
pub mod predict;
pub mod completions;
pub mod configuration;

use super::completion_handler::CompletionProcess;
pub use analyze::AnalyzeCluster;
pub use predict::PredictCost;
pub use cmdctl::*;
pub use configuration::Configurations;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    global_settings = &[DisableVersion, DeriveDisplayOrder, VersionlessSubcommands],
)]
pub enum Commands {
    /// Analyze cluster cost.
    #[structopt(name = "analyze")]
    AnalyzeCluster(AnalyzeCluster),

    /// Predict cost.
    ///
    ///
    #[structopt(name = "predict")]
    PredictCost(PredictCost),

    /// Configuration options.
    #[structopt(name = "configuration")]
    Configurations(Configurations),
}

impl Commands {
    pub fn process(&self) {
        match self {
            Commands::AnalyzeCluster(_) => {
                //
            },
            Commands::PredictCost(_) => {
                //
            },
            Commands::Configurations(config) => {
                match config {
                    Configurations::AppConfiguration(app_config) => {
                        //
                    },
                    Configurations::Completions(completion) => {
                        CompletionProcess::run(completion.to_owned());
                    },
                }
            },
        }
    }

    pub fn is_verbose(&self) -> bool {
        match self {
            Commands::AnalyzeCluster(cluster) => {
                cluster.verbose
            },
            Commands::PredictCost(predict) => {
                false
            },
            Commands::Configurations(_) => {
                false
            },
        }
    }
}
