use clap::ArgMatches;

use crate::terraform;
use crate::tools::kubectl::KubeCtl;

pub fn command(matches: Option<&ArgMatches>) {
    let _matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for bind command"),
    };

    terraform::commands::init();
    terraform::commands::apply();

    let kubectl = KubeCtl::default();
    kubectl.apply().expect("kubectl apply failed");
}
