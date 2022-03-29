use clap::Subcommand;

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Add an environment variable
    Add {
        /// The environment variable key
        key: String,
        /// The environment variable value
        value: String,
        /// The channel ID this environment variable will be bound to
        channel_id: String,
    },
    /// Remove an environment variable
    Remove {
        /// The environment variable ID
        id: String,
    },
}