use clap::ArgMatches;

pub type CommandFn = fn(Option<&ArgMatches>);

pub mod init;
pub mod compile;
pub mod deploy;
