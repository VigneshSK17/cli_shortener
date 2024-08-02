use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ClapArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,

    /// Increase verbosity level to see everything that's going on
    #[clap(short, long, action)]
    pub verbose: bool,

    /// Set the specific port for the web server
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Set the specific host IP addr
    #[clap(long, default_value = "127.0.0.1")]
    pub host: String,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create a new shortened link
    New(NewCommand),

    /// Delete a shortened link
    Delete(DeleteCommand),

    /// Lists all active shortened links
    List,
    /// Starts the web server which redirects the shortened links
    Start,
}

#[derive(Debug, Args)]
pub struct NewCommand {
    /// Link to be converted to a shortened link
    pub link: String,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    /// Link to be converted to a shortened link
    pub link: String,
}
