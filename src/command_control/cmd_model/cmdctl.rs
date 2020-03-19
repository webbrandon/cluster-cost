use structopt::StructOpt;
use structopt::clap::AppSettings::*;
use super::Commands;

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(
    global_settings = &[DisableVersion, DeriveDisplayOrder, VersionlessSubcommands],
    about = "Analyze and predict operating cost for a service in Kubernetes on AWS."
)]
pub struct CmdCtl {
    /// Daemon mode.
    #[structopt(short = "d", long = "daemon")]
    pub daemon: bool,

    /// Daemon mode port.
    #[structopt(long = "port", env = "OrderItem_PORT", default_value="8080")]
    pub port: i32,

    /// Daemon mode host.
    #[structopt(long = "host", env = "OrderItem_HOST", default_value="127.0.0.1")]
    pub host: String,

    /// Don't run commands only log.
    #[structopt(long = "dry-run")]
    pub dry_run: bool,

    /// Enable verbose logging.
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub commands: Option<Commands>,

}

impl CmdCtl {

    pub fn run_command_process(self) -> CmdCtl {
        match &self.commands {
            Some(commands) => {
                commands.process();
                self
            },
            None => {
                self
            },
        }
    }

    pub fn is_verbose(&self) -> bool {
        match self.commands.clone() {
            Some(commands) => commands.is_verbose(),
            None => self.verbose
        }
    }

}
