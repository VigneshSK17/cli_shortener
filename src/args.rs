use clap::{Args, Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ClapArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,

    /// Increase verbosity level to see everything that's going on
    #[clap(short, long, action)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create a new shortened link
    New(NewCommand),

    /// Delete a shortened link
    Delete(DeleteCommand),

    /// Deletes all existing shortened links
    Clear,
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