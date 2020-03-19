use structopt::StructOpt;

#[derive(StructOpt, Debug, Default, Clone)]
pub struct AnalyzeCluster {
    /// Filter container list. ( ie: my-deployment-name )
    #[structopt(short = "f", long = "filter")]
    pub filter: Option<String>,

    /// Namespace target. ( ie: Environment )
    #[structopt(short = "n", long = "namespace")]
    pub namespace: Option<String>,

    /// Cluster target.
    #[structopt(short = "c", long = "context")]
    pub context: Option<String>,

    /// Update token for eks using aws profile.
    #[structopt(long = "eks")]
    pub eks: Option<String>,

    /// Don't run commands only log.
    #[structopt(long = "dry-run")]
    pub dry_run: bool,

    /// Enable verbose logging.
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,
}
