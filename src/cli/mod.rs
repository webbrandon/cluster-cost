use crate::config_file::ConfigurationControl;
use crate::cost;
use crate::cluster;
use crate::cluster_cost;
use crate::cluster_print;
use crate::command_control;
use crate::toolbelt::logs::RootLog;
use structopt::StructOpt;
use crate::command_control::Commands;

pub struct Cli { }

impl Cli {
    pub async fn run_as_cli(config: ConfigurationControl) {
        // This is the collection of settings sent from the request.
        let cli_options = command_control::CmdCtl::from_args();

        // This should be passed to any underlying modules and follow verbose logic rules for CLI.
        let log_config = RootLog::get_logger(
            cli_options.is_verbose()
        );

        match &cli_options.commands {
            Some(commands) => {
                match commands {
                    Commands::AnalyzeCluster(analyze_this) => {
                        if let Ok(deployments) = cluster::ClusterRunner::run(&analyze_this).await {
                            if let Ok(analysis) = cluster_cost::ClusterCostRunner::run(&config, &deployments).await {
                                cluster_print::ClusterPrintRunner::run(&analysis).await;
                            }
                        }
                    },
                    Commands::PredictCost(predict_this) => {
                        if let Ok(cost_item) = cost::CostRunner::run(
                            &config,
                            &predict_this).await {
                                info!(log_config,
"-------------------------------------
Predicted Cost
-------------------------------------
nodeType: {}
cpu reserved: {}
memory reserved: {}
monthly: ${:.2}
weekly: ${:.2}
daily: ${:.2}
hourly: ${:.2}\n\n",
                                    cost_item.node_type.unwrap(),
                                    cost_item.cpu.unwrap(),
                                    cost_item.memory.unwrap(),
                                    cost_item.monthly.unwrap(),
                                    cost_item.weekly.unwrap(),
                                    cost_item.daily.unwrap(),
                                    cost_item.hourly.unwrap()
                                );
                        }
                    },
                    Commands::Configurations(_) => {
                        cli_options.clone().run_command_process();
                    }
                }
            },
            None => {
                // come back attach cargo.toml version
                info!(log_config, "cluster-cost v{}", "0.1.0")
            }
        }
    }
}
