#[allow(unused_imports)]
#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;
extern crate serde;
extern crate serde_yaml;
extern crate serde_json;
extern crate env_logger;
extern crate run_script;
extern crate regex;
extern crate dialoguer;
extern crate console;
extern crate dirs;
extern crate actix_web;
extern crate actix_rt;
extern crate tokio;
extern crate structopt;
extern crate serde_value;

mod cli;
mod daemon;
mod config_file;
mod command_control;
mod cluster_cost;
mod cluster_print;
mod cluster;
mod cost;
mod toolbelt;
mod prompt_user;

use cli::Cli;
use daemon::Daemeon;
use structopt::StructOpt;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // This is the collection of settings sent from the request.
    let cli_options = command_control::CmdCtl::from_args();

    let config_control = config_file::ConfigurationControl::new().load();

    if !cli_options.daemon {
        Cli::run_as_cli(config_control).await;
        Ok(())
    } else {
        let mut dameon_client = Daemeon::new(config_control);
        dameon_client.run_as_daemeon().await
    }
}
