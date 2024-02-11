use assemblylift_tools::terraform::Terraform;
use clap::ArgMatches;

pub fn command(matches: Option<&ArgMatches>) {
    let _matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for bind command"),
    };

    let tf = Terraform::default();
    tf.init();
    tf.apply();
}
