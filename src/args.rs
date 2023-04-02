use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ClapArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
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
