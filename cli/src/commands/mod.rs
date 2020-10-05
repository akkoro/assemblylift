use clap::ArgMatches;

pub type CommandFn = fn(Option<&ArgMatches>);

pub mod bind;
pub mod burn;
pub mod cast;
pub mod init;
pub mod make;
