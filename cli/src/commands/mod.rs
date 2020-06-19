use clap::ArgMatches;

pub type CommandFn = fn(Option<&ArgMatches>);

pub mod init;
pub mod cast;
pub mod bind;
